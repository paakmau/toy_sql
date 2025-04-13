use std::str::FromStr;

use chumsky::{
    IterParser, Parser,
    extra::ParserExtra,
    prelude::{any, just, none_of},
    span::SimpleSpan,
};
use strum::IntoEnumIterator;

use super::token::{Keyword, Symbol, Token};

pub fn lexer<'src, E>() -> impl Parser<'src, &'src str, Vec<(Token<'src>, SimpleSpan)>, E>
where
    E: ParserExtra<'src, &'src str>,
{
    let raw_string = none_of('\'')
        .repeated()
        .to_slice()
        .map(|raw| Token::String { raw })
        .padded_by(just('\''));

    let word = any()
        .filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
        .then(
            any()
                .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                .repeated(),
        )
        .to_slice()
        .map(|raw| {
            if let Ok(keyword) = Keyword::from_str(raw) {
                Token::Keyword(keyword)
            } else {
                Token::Ident { raw }
            }
        });

    let symbol = any()
        .filter(|c: &char| Symbol::iter().any(|s| s as u8 == *c as u8))
        .map(|c: char| Token::Symbol(Symbol::from_repr(c as u8).unwrap()));

    let token = raw_string.or(word).or(symbol);

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded()
        .repeated()
        .collect()
}
