use nom::{bytes::complete::take_while, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableName(String);

pub fn variable_name(input: &str) -> IResult<&str, VariableName> {
    let (input, name) = take_while(|c: char| c.is_ascii_alphabetic() || c == ' ')(input)?;

    Ok((input, VariableName(name.to_owned())))
}
