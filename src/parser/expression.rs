use nom::IResult;

use crate::parser::{
    equality::{equality, Equality},
    literal::{literal, Literal},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Equality(Equality),
    Literal(Literal),
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    if let Ok((input, lit)) = equality(input) {
        Ok((input, Expression::Equality(lit)))
    } else {
        let (input, eq) = literal(input)?;
        Ok((input, Expression::Literal(eq)))
    }
}
