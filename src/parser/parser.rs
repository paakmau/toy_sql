use chumsky::{
    Parser, extra::ParserExtra, input::BorrowInput, prelude::just, select_ref, span::SimpleSpan,
};

use super::token::{Keyword, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<'src> {
    ident: &'src str,
}

pub fn tree<'src, I, E>() -> impl Parser<'src, I, (TreeNode<'src>, SimpleSpan), E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    just(Token::Keyword(Keyword::Select))
        .ignore_then(select_ref! { Token::Ident { raw } => raw })
        .map(|ident| TreeNode { ident })
        .map_with(|tok, e| (tok, e.span()))
}
