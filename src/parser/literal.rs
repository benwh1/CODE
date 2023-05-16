use nom::{bytes::complete::take_while1, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(IntegerLit),
    String(StringLit),
}

pub fn literal(input: &str) -> IResult<&str, Literal> {
    if let Ok((input, lit)) = integer(input) {
        Ok((input, Literal::Integer(lit)))
    } else {
        let (input, lit) = string(input)?;
        Ok((input, Literal::String(lit)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLit(pub(crate) i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLit(pub(crate) String);

pub fn integer(input: &str) -> IResult<&str, IntegerLit> {
    let (input, lit) = take_while1(|c: char| c.is_ascii_digit())(input)?;

    Ok((input, IntegerLit(lit.parse().unwrap())))
}

pub fn string(input: &str) -> IResult<&str, StringLit> {
    Ok(("", StringLit(input.to_owned())))
}
