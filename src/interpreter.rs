use std::{collections::HashMap, fmt::Display};

use crate::parser::{
    binary_op::Operation,
    expression::Expression,
    identifier::Identifier,
    literal::{IntegerLit, Literal, StringLit},
    program::Program,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    String,
    Custom(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value.as_ref() {
            "int" => Self::Integer,
            "string" => Self::String,
            _ => Self::Custom(value.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i8),
    String(String),
    Uninitialized,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'a> From<&'a Literal> for Value {
    fn from(value: &'a Literal) -> Self {
        match value {
            Literal::Integer(IntegerLit(n)) => Self::Integer(*n as i8),
            Literal::String(StringLit(s)) => Self::String(s.to_owned()),
        }
    }
}

impl Value {
    pub fn to_integer(&self) -> i8 {
        match self {
            Value::Integer(n) => *n,
            Value::String(s) => s.parse().unwrap_or(127),
            Value::Uninitialized => 127,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Uninitialized => String::from("nothing"),
        }
    }

    pub fn cast(&mut self, to: Type) {
        match to {
            Type::Integer => *self = Value::Integer(self.to_integer()),
            Type::String => *self = Value::String(self.to_string()),
            Type::Custom(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: Identifier,
    r#type: Type,
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
                    rhs.cast(var.r#type.clone());
                    var.value = rhs.clone();

                    rhs
                } else {
                    // If the RHS is an identifier, use it as the type name
                    let var_type = match eq.rhs.as_ref() {
                        Expression::Identifier(ident) => Type::from(ident.0.as_ref()),
                        _ => todo!(),
                    };
                    let var = Variable {
                        name: eq.lhs.clone(),
                        r#type: var_type,
                        value: Value::Uninitialized,
                    };
                    self.variables.insert(eq.lhs.clone(), var);
                    Value::Uninitialized
                }
            }
            Expression::ComeFrom(_) => Value::Integer(0),
            Expression::Print(p) => {
                let value = self.eval_expression(&p.0);
                println!("{value}");
                value
            }
            Expression::BinaryOp(op) => {
                let lhs = self.eval_expression(&op.lhs).to_integer();
                let rhs = self.eval_expression(&op.rhs).to_integer();
                Value::Integer(match op.op {
                    Operation::Add => lhs.wrapping_add(rhs),
                    Operation::Sub => lhs.wrapping_sub(rhs),
                    Operation::Mul => lhs.wrapping_mul(rhs),
                    Operation::Div => lhs.wrapping_div(rhs),
                    Operation::Mod => lhs.wrapping_rem(rhs),
                })
            }
            Expression::Identifier(ident) => {
                if let Some(var) = self.variables.get(ident) {
                    var.value.clone()
                } else {
                    Value::Uninitialized
                }
            }
            Expression::Literal(lit) => lit.into(),
        }
    }
}
