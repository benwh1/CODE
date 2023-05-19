use nom::{
    bytes::{complete::take_while1, streaming::tag},
    error::{Error, ErrorKind},
    Err, IResult,
};

use crate::parser_chain;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(IntegerLit),
    String(StringLit),
}

pub fn literal(input: &str, use_all_input: bool) -> IResult<&str, Literal> {
    parser_chain!(
        |i| integer(i).map(|(input, lit)| (input, Literal::Integer(lit))),
        |i| string(i).map(|(input, lit)| (input, Literal::String(lit)));
        input,
        use_all_input
    );

    Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLit(pub(crate) i128);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLit(pub(crate) String);

pub fn integer(input: &str) -> IResult<&str, IntegerLit> {
    let (input, lit) = take_while1(|c: char| c.is_ascii_digit())(input)?;

    Ok((input, IntegerLit(lit.parse().unwrap())))
}

pub fn string(input: &str) -> IResult<&str, StringLit> {
    let (input, _) = tag("the string ")(input)?;

    Ok(("", StringLit(input.to_owned())))
}
