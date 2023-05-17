use nom::{bytes::streaming::tag, IResult};

use crate::parser::line::literal::{integer, IntegerLit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComeFrom {
    pub(crate) line_number: IntegerLit,
}

pub fn come_from(input: &str) -> IResult<&str, ComeFrom> {
    let (input, _) = tag("come from ")(input)?;
    let (input, line_number) = integer(input)?;

    Ok((input, ComeFrom { line_number }))
}
