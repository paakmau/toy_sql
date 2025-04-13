use chumsky::{Parser, extra::ParserExtra, input::BorrowInput, span::SimpleSpan};

use super::{
    ast::select::{self, SelectClause},
    token::Token,
};

pub fn parser<'src, I, E>() -> impl Parser<'src, I, SelectClause, E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    select::parser()
}
