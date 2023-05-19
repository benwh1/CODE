pub mod int;
pub mod z;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::{
    interpreter::{
        r#type::Type,
        value::{int::Int, z::Z},
    },
    parser::line::literal::{IntegerLit, Literal, StringLit},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(Int),
    Z(Z),
    String(String),
    Uninitialized(Type),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => f.write_str(&n.0.to_string()),
            Value::Z(z) => f.write_str(&z.0.to_string()),
            Value::String(s) => f.write_str(s),
            Value::Uninitialized(_) => f.write_str("nothing"),
        }
    }
}

impl<'a> From<&'a Literal> for Value {
    fn from(value: &'a Literal) -> Self {
        match value {
            Literal::Integer(IntegerLit(n)) => {
                let n = *n;
                if let Ok(n) = n.try_into() {
                    Self::Int(Int(n))
                } else {
                    Self::Z(Z(n))
                }
            }
            Literal::String(StringLit(s)) => Self::String(s.to_owned()),
        }
    }
}

impl Value {
    pub fn r#type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Z(_) => Type::Z,
            Value::String(_) => Type::String,
            Value::Uninitialized(t) => t.clone(),
        }
    }

    pub fn to_int(&self) -> Int {
        match self {
            Value::Int(n) => *n,
            Value::Z(z) => Int(z.0.try_into().unwrap_or(127)),
            Value::String(s) => s.parse().unwrap_or(Int(127)),
            Value::Uninitialized(_) => Int(127),
        }
    }

    pub fn to_z(&self) -> Z {
        match self {
            Value::Int(n) => Z(n.0 as i128),
            Value::Z(z) => *z,
            Value::String(s) => s.parse().unwrap_or(Z(i128::MAX)),
            Value::Uninitialized(_) => Z(i128::MAX),
        }
    }

    pub fn cast(&mut self, to: Type) {
        match to {
            Type::Int => *self = Value::Int(self.to_int()),
            Type::Z => *self = Value::Z(self.to_z()),
            Type::String => *self = Value::String(self.to_string()),
            Type::Custom(_) => todo!(),
        }
    }

    pub fn modular_div(self, rhs: Self) -> Self {
        match self {
            Self::Int(n) => Self::Int(n.modular_div(rhs.to_int())),
            Self::Z(_) => todo!(),
            Self::String(ref s) => {
                // If s is an integer string, convert it to an int
                if let Ok(n) = s.parse() {
                    Self::String((Self::Int(n).modular_div(rhs)).to_string())
                } else {
                    self / rhs
                }
            }
            Self::Uninitialized(_) => self,
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(n) => Self::Int(n + rhs.to_int()),
            Self::Z(z) => Self::Z(z + rhs.to_z()),
            Self::String(mut s) => {
                match rhs {
                    Self::String(s2) => {
                        // If both strings are numeric, cast them to integers and add them
                        // (try Int first, then Z)
                        if let (Ok(a), Ok(b)) = (s.parse::<Int>(), s2.parse()) {
                            Self::String((a + b).0.to_string())
                        } else if let (Ok(a), Ok(b)) = (s.parse::<Z>(), s2.parse()) {
                            Self::String((a + b).0.to_string())
                        } else {
                            s.push_str(&s2);
                            Self::String(s)
                        }
                    }
                    Self::Int(_) | Self::Z(_) | Self::Uninitialized(_) => {
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
            Self::Int(n) => Self::Int(n - rhs.to_int()),
            Self::Z(z) => Self::Z(z - rhs.to_z()),
            Self::String(mut s) => match rhs {
                Self::Int(_) => Self::String(s) / Self::Z(rhs.to_z()),
                Self::Z(Z(z)) => {
                    // Remove the last `z` characters from the string
                    for _ in 0..z {
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

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(n) => Self::Int(n * rhs.to_int()),
            Self::Z(z) => Self::Z(z * rhs.to_z()),
            Self::String(s) => {
                // If s is an integer string, convert it to an int or z
                if let Ok(n) = s.parse() {
                    Self::String((Self::Int(n) * rhs).to_string())
                } else if let Ok(z) = s.parse() {
                    Self::String((Self::Z(z) * rhs).to_string())
                } else {
                    Self::String(s.repeat(rhs.to_int().0 as usize))
                }
            }
            Self::Uninitialized(_) => self,
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(n) => Self::Int(n / rhs.to_int()),
            Self::Z(z) => Self::Z(z / rhs.to_z()),
            Self::String(s) => {
                // If s is an integer string, convert it to an int or z
                if let Ok(n) = s.parse() {
                    Self::String((Self::Int(n) / rhs).to_string())
                } else if let Ok(z) = s.parse() {
                    Self::String((Self::Z(z) / rhs).to_string())
                } else {
                    let new_len = s.chars().count() / rhs.to_int().0 as usize;
                    Self::String(s.chars().take(new_len).collect())
                }
            }
            Self::Uninitialized(_) => self,
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut v = Self::Int(self.to_int() % rhs.to_int());
        v.cast(self.r#type());
        v
    }
}
