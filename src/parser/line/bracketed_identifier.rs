use nom::{
    character::complete::char,
    multi::{count, many0_count},
    IResult,
};

use crate::parser::line::identifier::{identifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketedIdentifier {
    pub(crate) identifier: Identifier,
    pub(crate) num_brackets: u32,
}

pub fn bracketed_identifier(input: &str) -> IResult<&str, BracketedIdentifier> {
    let (input, cnt_left) = many0_count(char('('))(input)?;
    let (input, ident) = identifier(input)?;
    let (input, _) = count(char(')'), cnt_left)(input)?;

    Ok((
        input,
        BracketedIdentifier {
            identifier: ident,
            num_brackets: cnt_left as u32,
        },
    ))
}
