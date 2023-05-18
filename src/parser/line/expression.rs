use nom::{
    error::{Error, ErrorKind},
    Err, IResult,
};

use crate::{
    parser::line::{
        binary_op::{binary_op, BinaryOp},
        come_from::{come_from, ComeFrom},
        equality::{equality, Equality},
        identifier::{identifier, Identifier},
        literal::{literal, Literal},
        print::{print, Print},
    },
    parser_chain,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // Conditionals are not parsed here because they have the same syntax as equality. We parse as
    // `Equality` first, and then `Program` converts the relevant `Equality`s to `Conditional`s.
    Conditional(Equality),
    Equality(Equality),
    ComeFrom(ComeFrom),
    Print(Print),
    BinaryOp(BinaryOp),
    Literal(Literal),
    Identifier(Identifier),
    None,
}

pub fn expression(input: &str, use_all_input: bool) -> IResult<&str, Expression> {
    if input == "" && use_all_input {
        return Ok(("", Expression::None));
    }

    parser_chain!(
        |i| equality(i).map(|(input, expr)| (input, Expression::Equality(expr))),
        |i| come_from(i).map(|(input, expr)| (input, Expression::ComeFrom(expr))),
        |i| print(i).map(|(input, expr)| (input, Expression::Print(expr))),
        |i| binary_op(i).map(|(input, expr)| (input, Expression::BinaryOp(expr))),
        |i| literal(i, use_all_input).map(|(input, expr)| (input, Expression::Literal(expr))),
        |i| identifier(i).map(|(input, expr)| (input, Expression::Identifier(expr)));
        input,
        use_all_input
    );

    Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
}
