use nom::IResult;

use crate::parser::{
    come_from::{come_from, ComeFrom},
    equality::{equality, Equality},
    literal::{literal, Literal},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Equality(Equality),
    ComeFrom(ComeFrom),
    Literal(Literal),
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    if let Ok((input, lit)) = equality(input) {
        Ok((input, Expression::Equality(lit)))
    } else if let Ok((input, cf)) = come_from(input) {
        Ok((input, Expression::ComeFrom(cf)))
    } else {
        let (input, eq) = literal(input)?;
        Ok((input, Expression::Literal(eq)))
    }
}
