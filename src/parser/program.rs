use crate::parser::line::indented_expression::{indented_expression, IndentedExpression};

use super::line::expression::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub(crate) expressions: Vec<IndentedExpression>,
}

pub fn program(input: &str) -> Program {
    let mut expressions = input
        .lines()
        .map(|l| indented_expression(l, true).unwrap().1)
        .collect::<Vec<_>>();

    // Go through the expressions and convert the required `Equality`s into `Conditional`s, based
    // on indentation
    for i in 0..expressions.len() {
        if let Ok([cur, next]) = expressions.get_many_mut([i, i + 1]) {
            if next.indent_depth > cur.indent_depth {
                if let Expression::Equality(e) = &cur.expr {
                    cur.expr = Expression::Conditional(e.clone());
                }
            }
        }
    }

    Program { expressions }
}
