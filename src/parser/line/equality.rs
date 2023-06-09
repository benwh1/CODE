use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use crate::parser::line::{
    bracketed_identifier::{bracketed_identifier, BracketedIdentifier},
    indented_expression::{indented_expression, IndentedExpression},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equality {
    pub(crate) lhs: BracketedIdentifier,
    pub(crate) rhs: Box<IndentedExpression>,
}

pub fn equality(input: &str) -> IResult<&str, Equality> {
    let Some((lhs, rhs)) = input.split_once(" = ") else {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    };

    let (result, lhs) = bracketed_identifier(lhs)?;
    if !result.is_empty() {
        return Err(nom::Err::Failure(Error::new(result, ErrorKind::Fail)));
    }

    let (result, rhs) = indented_expression(rhs, true)?;
    if !result.is_empty() {
        return Err(nom::Err::Failure(Error::new(result, ErrorKind::Fail)));
    }

    Ok((
        "",
        Equality {
            lhs,
            rhs: Box::new(rhs),
        },
    ))
}
