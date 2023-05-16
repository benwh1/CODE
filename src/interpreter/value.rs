use std::fmt::Display;

use crate::{
    interpreter::r#type::Type,
    parser::literal::{IntegerLit, Literal, StringLit},
};

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
