use nom::{bytes::complete::take_while, IResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub(crate) String);

pub fn identifier(input: &str) -> IResult<&str, Identifier> {
    let (input, name) = take_while(|c: char| c.is_ascii_alphabetic() || c == ' ')(input)?;

    Ok((input, Identifier(name.to_owned())))
}
