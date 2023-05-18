use std::{collections::HashMap, matches};

use crate::{
    interpreter::{int::Int, r#type::Type, value::Value},
    parser::{
        line::{
            binary_op::Operation, bracketed_identifier::BracketedIdentifier,
            expression::Expression, identifier::Identifier,
        },
        program::Program,
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
            .filter_map(|(idx, e)| match &e.expr {
                Expression::ComeFrom(c) => Some((c.line_number.0 as usize - 1, idx)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        let mut current_line = 0;
        while current_line < program.expressions.len() {
            let expr = &program.expressions[current_line];
            let value = self.eval_expression(&expr.expr);

            if let Some(&to) = come_froms.get(&current_line) {
                current_line = to;
            } else if matches!(expr.expr, Expression::Conditional(_))
                && value == Value::Integer(Int(0))
            {
                // If we just evaluated a conditional to false, skip over the indented block
                loop {
                    current_line += 1;
                    if let Some(e) = program.expressions.get(current_line) {
                        if e.indent_depth <= expr.indent_depth {
                            break;
                        }
                    } else {
                        return;
                    }
                }
            } else {
                // Otherwise, just go to the next line
                current_line += 1;
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Conditional(eq) => {
                let rhs = self.eval_expression(&eq.rhs);

                // Value of the LHS variable, or an uninitialized int if it's not defined
                let mut lhs = self
                    .resolve_bracketed_identifier(&eq.lhs)
                    .and_then(|ident| self.variables.get(&ident))
                    .map(|v| &v.value)
                    .unwrap_or_else(|| &Value::Uninitialized(Type::Integer))
                    .to_owned();
                lhs.cast(rhs.r#type());

                Value::Integer(Int((lhs == rhs) as u8))
            }
            Expression::Equality(eq) => {
                let mut rhs = self.eval_expression(&eq.rhs);

                let Some(ident) = self.resolve_bracketed_identifier(&eq.lhs) else {
                    // The LHS of the equality is invalid. Set the inner identifier to 127.
                    self.set_variable_or_create(eq.lhs.identifier.clone(), Value::Integer(Int(127)));
                    return Value::Integer(Int(127));
                };

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
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                    Operation::ModularDiv => lhs.modular_div(rhs),
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

    pub fn resolve_bracketed_identifier(
        &self,
        bracketed: &BracketedIdentifier,
    ) -> Option<Identifier> {
        // (x) = y means set the variable whose name is the value of x, to y.
        // So if LHS is in brackets, `eval` it that many times.
        let mut ident = bracketed.identifier.clone();
        for _ in 0..bracketed.num_brackets {
            if let Some(var) = self.variables.get(&ident) {
                ident = Identifier(var.value.to_string());
            } else {
                return None;
            }
        }

        Some(ident)
    }
}
