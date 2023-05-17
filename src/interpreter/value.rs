use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::{
    interpreter::r#type::Type,
    parser::literal::{IntegerLit, Literal, StringLit},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i8),
    String(String),
    Uninitialized(Type),
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
    pub fn r#type(&self) -> Type {
        match self {
            Value::Integer(_) => Type::Integer,
            Value::String(_) => Type::String,
            Value::Uninitialized(t) => t.clone(),
        }
    }

    pub fn to_integer(&self) -> i8 {
        match self {
            Value::Integer(n) => *n,
            Value::String(s) => s.parse().unwrap_or(127),
            Value::Uninitialized(_) => 127,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Uninitialized(_) => String::from("nothing"),
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

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(n) => Self::Integer(n.wrapping_add(rhs.to_integer())),
            Self::String(mut s) => {
                match rhs {
                    Self::String(s2) => {
                        // If both strings are numeric, cast them to integers and add them
                        if let (Ok(a), Ok(b)) = (s.parse::<i8>(), s2.parse::<i8>()) {
                            Self::String((a + b).to_string())
                        } else {
                            s.push_str(&s2);
                            Self::String(s)
                        }
                    }
                    Self::Integer(_) | Self::Uninitialized(_) => {
                        Self::String(format!("{s}{rhs}", rhs = rhs.to_string()))
                    }
                }
            }
            Self::Uninitialized(_) => Self::Uninitialized(rhs.r#type()),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(n) => Self::Integer(n.wrapping_sub(rhs.to_integer())),
            Self::String(mut s) => match rhs {
                Self::Integer(n) => {
                    // If n is nonnegative, remove the last n characters from s
                    for _ in 0..n {
                        s.pop();
                    }
                    Self::String(s)
                }
                Self::String(_) | Self::Uninitialized(_) => {
                    let s2 = rhs.to_string();

                    // Remove all instances of `s2` from `s` and collect into a new string
                    Self::String(s.split(&s2).collect())
                }
            },
            Self::Uninitialized(_) => Self::Uninitialized(self.r#type()),
        }
    }
}
