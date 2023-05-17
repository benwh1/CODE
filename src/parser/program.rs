use crate::parser::line::expression::{expression, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub(crate) expressions: Vec<Expression>,
}

pub fn program(input: &str) -> Program {
    Program {
        expressions: input
            .lines()
            .map(|l| expression(l, true).unwrap().1)
            .collect(),
    }
}
