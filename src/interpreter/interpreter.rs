use std::collections::HashMap;

use crate::{
    interpreter::{int::Int, r#type::Type, value::Value},
    parser::{
        binary_op::Operation, expression::Expression, identifier::Identifier, program::Program,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: Identifier,
    value: Value,
}

impl Variable {
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
}

pub struct InterpreterState {
    variables: HashMap<Identifier, Variable>,
}

impl InterpreterState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Program) {
        // Hashmap of `from -> to` rules
        let come_froms = program
            .expressions
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| match e {
                Expression::ComeFrom(c) => Some((c.line_number.0 as usize - 1, idx)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        let mut current_line = 0;
        while current_line < program.expressions.len() {
            let expr = &program.expressions[current_line];
            self.eval_expression(expr);

            if let Some(&to) = come_froms.get(&current_line) {
                current_line = to;
            } else {
                current_line += 1;
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Equality(eq) => {
                let mut rhs = self.eval_expression(&eq.rhs);

                // (x) = y means set the variable whose name is the value of x, to y.
                // So if LHS is in brackets, `eval` it that many times.
                let mut ident = eq.lhs.identifier.clone();
                for _ in 0..eq.lhs.num_brackets {
                    if let Some(var) = self.variables.get(&ident) {
                        ident = Identifier(var.value.to_string());
                    } else {
                        // The LHS of the equality is invalid. Set the inner identifier to 127.
                        self.set_variable_or_create(ident, Value::Integer(Int(127)));
                        return Value::Integer(Int(127));
                    }
                }

                // If the identifier is a variable name, set the value of the variable
                // Otherwise, create a variable with RHS as the type
                if let Some(var) = self.variables.get_mut(&ident) {
                    rhs.cast(var.value.r#type());
                    var.set_value(rhs.clone());

                    rhs
                } else {
                    // If the RHS is an identifier, use it as the type name
                    let var_type = match eq.rhs.as_ref() {
                        Expression::Identifier(ident) => Type::from(ident.0.as_ref()),
                        _ => todo!(),
                    };
                    let value = Value::Uninitialized(var_type);
                    self.create_variable(ident, value.clone());
                    value
                }
            }
            Expression::ComeFrom(_) => Value::Integer(Int(0)),
            Expression::Print(p) => {
                let value = self.eval_expression(&p.0);
                println!("{value}");
                value
            }
            Expression::BinaryOp(op) => {
                let lhs = self.eval_expression(&op.lhs);
                let rhs = self.eval_expression(&op.rhs);
                match op.op {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => todo!(),
                    Operation::Div => todo!(),
                    Operation::Mod => todo!(),
                }
            }
            Expression::Identifier(ident) => {
                if let Some(var) = self.variables.get(ident) {
                    var.value.clone()
                } else {
                    Value::Uninitialized(Type::Integer)
                }
            }
            Expression::Literal(lit) => lit.into(),
        }
    }

    pub fn create_variable(&mut self, name: Identifier, value: Value) {
        self.variables
            .insert(name.clone(), Variable { name, value });
    }

    pub fn set_variable_or_create(&mut self, name: Identifier, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            var.set_value(value);
        } else {
            self.create_variable(name, value);
        }
    }
}
