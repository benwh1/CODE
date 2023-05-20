use std::ops::{Add, Mul};

use crate::interpreter::value::z::Z;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Function {
    // List of line numbers
    pub(crate) lines: Vec<usize>,
}

impl Add for Function {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.lines.append(&mut rhs.lines);
        self
    }
}

impl Mul<Z> for Function {
    type Output = Self;

    fn mul(self, rhs: Z) -> Self::Output {
        Self {
            lines: self.lines.repeat(rhs.0 as usize),
        }
    }
}
