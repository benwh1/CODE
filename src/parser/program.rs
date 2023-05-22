use crate::parser::line::indented_expression::{indented_expression, IndentedExpression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub(crate) expressions: Vec<IndentedExpression>,
}

pub fn program(input: &str) -> Program {
    Program {
        expressions: input
            .lines()
            .map(|l| indented_expression(l, true).unwrap().1)
            .collect::<Vec<_>>(),
    }
}
