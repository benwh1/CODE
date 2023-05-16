use crate::parser::expression::{expression, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub(crate) expressions: Vec<Expression>,
}

pub fn program(input: &str) -> Program {
    Program {
        expressions: input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let (rest, expr) = expression(l).unwrap();
                if rest != "" {
                    panic!("Syntax error on line {}", i + 1);
                }
                expr
            })
            .collect(),
    }
}
