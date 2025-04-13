use chumsky::{
    Parser, extra::ParserExtra, input::BorrowInput, prelude::just, select_ref, span::SimpleSpan,
};

use crate::parser::token::{Symbol, Token};

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Field(String),
    BinaryOp {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

pub fn parser<'src, I, E>() -> impl Parser<'src, I, Expr, E>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    E: ParserExtra<'src, I>,
{
    let string = select_ref!(Token::String { raw } => raw).map(|raw| Expr::String(raw.to_string()));

    let ident =
        select_ref!(Token::Ident { raw } => raw).map(|raw| Expr::Field(raw.to_ascii_lowercase()));

    let atom = string.or(ident);

    let compare_op = just(Token::Symbol(Symbol::Equals))
        .to(Operator::Equals)
        .or(just(Token::Symbol(Symbol::LessThan)).to(Operator::LessThan))
        .or(just(Token::Symbol(Symbol::GreaterThan)).to(Operator::GreaterThan));

    atom.clone()
        .then(compare_op)
        .then(atom)
        .map(|((lhs, op), rhs)| Expr::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
}
