use nom::{bytes::complete::take_while, IResult};

pub enum Literal {
    Integer(IntegerLit),
    String(StringLit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLit(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLit(String);

pub fn parse_integer(input: &str) -> IResult<&str, IntegerLit> {
    let (input, lit) = take_while(|c: char| c.is_ascii_digit())(input)?;

    Ok((input, IntegerLit(lit.parse().unwrap())))
}

pub fn parse_string(input: &str) -> IResult<&str, StringLit> {
    Ok((input, StringLit(input.to_owned())))
}
