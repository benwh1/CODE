use std::collections::HashMap;

use crate::{
    interpreter::{r#type::Type, value::Value},
    parser::{
        binary_op::Operation, expression::Expression, identifier::Identifier, program::Program,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: Identifier,
    value: Value,
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

                // If LHS is a variable name, set the value of the variable
                // Otherwise, create a variable with RHS as the type
                if let Some(var) = self.variables.get_mut(&eq.lhs) {
                    rhs.cast(var.value.r#type());
                    var.value = rhs.clone();

                    rhs
                } else {
                    // If the RHS is an identifier, use it as the type name
                    let var_type = match eq.rhs.as_ref() {
                        Expression::Identifier(ident) => Type::from(ident.0.as_ref()),
                        _ => todo!(),
                    };
                    let value = Value::Uninitialized(var_type);
                    let var = Variable {
                        name: eq.lhs.clone(),
                        value: value.clone(),
                    };
                    self.variables.insert(eq.lhs.clone(), var);
                    value
                }
            }
            Expression::ComeFrom(_) => Value::Integer(0),
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
                    Operation::Sub => todo!(),
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
}
