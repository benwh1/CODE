use nom::{character::complete::char, multi::many0_count, IResult};

use crate::parser::line::expression::{expression, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedExpression {
    pub(crate) expr: Expression,
    pub(crate) indent_depth: u32,
}

pub fn indented_expression(input: &str, use_all_input: bool) -> IResult<&str, IndentedExpression> {
    let (input, n) = many0_count(char(' '))(input)?;
    let (_, expr) = expression(input, use_all_input)?;

    Ok((
        "",
        IndentedExpression {
            expr,
            indent_depth: n as u32,
        },
    ))
}
