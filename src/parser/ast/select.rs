use chumsky::{
    IterParser, Parser, extra::ParserExtra, input::BorrowInput, prelude::just, select_ref,
    span::SimpleSpan,
};

use crate::parser::token::{Keyword, Symbol, Token};

use super::expr;

pub fn parser<'src, I, E>() -> impl Parser<'src, I, SelectClause, E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    select_clause()
        .then_ignore(just(Token::Keyword(Keyword::From)))
        .then_ignore(
            select_ref!(Token::Ident { raw } => raw)
                .separated_by(just(Token::Symbol(Symbol::Comma)))
                .at_least(1),
        )
        .then_ignore(just(Token::Keyword(Keyword::Where)))
        .then_ignore(expr())
        .map(|select_clause| select_clause)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Wildcard,
    NamedExpr {
        expr: expr::Expr,
        alias: Option<String>,
    },
}

fn expr<'src, I, E>() -> impl Parser<'src, I, Expr, E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    let wildcard = just(Token::Symbol(Symbol::Asterisk)).to(Expr::Wildcard);

    let named_expr = expr::parser()
        .then_ignore(just(Token::Keyword(Keyword::As)).repeated().at_most(1))
        .then(select_ref!(Token::Ident { raw } => raw))
        .map(|(expr, alias)| Expr::NamedExpr {
            expr,
            alias: Some(alias.to_ascii_lowercase()),
        })
        .or(expr::parser().map(|expr| Expr::NamedExpr { expr, alias: None }));

    wildcard.or(named_expr)
}

#[derive(Debug, Clone)]
pub struct SelectClause {
    pub projection: Vec<Expr>,
}

fn select_clause<'src, I, E>() -> impl Parser<'src, I, SelectClause, E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    just(Token::Keyword(Keyword::Select))
        .ignore_then(
            expr()
                .separated_by(just(Token::Symbol(Symbol::Comma)))
                .at_least(1)
                .collect::<Vec<Expr>>(),
        )
        .map(|exprs| SelectClause { projection: exprs })
}
