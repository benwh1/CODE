use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use crate::parser::{
    expression::{expression, Expression},
    identifier::{identifier, Identifier},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equality {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Box<Expression>,
}

pub fn equality(input: &str) -> IResult<&str, Equality> {
    let Some((lhs, rhs)) = input.split_once(" = ") else {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    };

    let (result, lhs) = identifier(lhs)?;
    if result != "" {
        return Err(nom::Err::Failure(Error::new(result, ErrorKind::Fail)));
    }

    let (result, rhs) = expression(rhs)?;
    if result != "" {
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
