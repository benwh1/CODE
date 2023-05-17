use nom::{bytes::streaming::tag, IResult};

use crate::parser::line::expression::{expression, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Print(pub(crate) Box<Expression>);

pub fn print(input: &str) -> IResult<&str, Print> {
    let (input, _) = tag("print ")(input)?;
    let (input, expr) = expression(input, true)?;

    Ok((input, Print(Box::new(expr))))
}
