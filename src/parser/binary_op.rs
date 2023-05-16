use nom::{
    error::{Error, ErrorKind},
    IResult,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::parser::expression::{expression, Expression};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Operation {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "÷",
            Self::Mod => "÷÷",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    pub(crate) lhs: Box<Expression>,
    pub(crate) rhs: Box<Expression>,
    pub(crate) op: Operation,
}

pub fn binary_op(input: &str) -> IResult<&str, BinaryOp> {
    // Find the operation that appears earliest in the string
    let Some((op, lhs, rhs)) = Operation::iter()
        .filter_map(|op| {
            input
                .split_once(&format!(" {} ", op.symbol()))
                .map(|(lhs, rhs)| (op, lhs, rhs))
        })
        .min_by_key(|(_, lhs, _)| lhs.len())
    else {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    };

    let (rest, lhs) = expression(lhs, true)?;
    if rest != "" {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    }

    let (rest, rhs) = expression(rhs, true)?;
    if rest != "" {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    }

    Ok((
        "",
        BinaryOp {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        },
    ))
}
