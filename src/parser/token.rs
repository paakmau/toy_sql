use strum::{EnumIter, EnumString, FromRepr};

#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Keyword {
    As,
    From,
    Insert,
    Into,
    Select,
    Where,
}

#[derive(EnumIter, FromRepr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Symbol {
    ExclamationMark = b'!',
    NumberSign = b'#',
    Dollar = b'$',
    Percent = b'%',
    Ampersand = b'&',
    LeftParenthesis = b'(',
    RightParenthesis = b')',
    Asterisk = b'*',
    Plus = b'+',
    Comma = b',',
    Minus = b'-',
    Period = b'.',
    Slash = b'/',
    Colon = b':',
    Semicolon = b';',
    LessThan = b'<',
    Equals = b'=',
    GreaterThan = b'>',
    QuestionMark = b'?',
    At = b'@',
    LeftBracket = b'[',
    Backslash = b'\\',
    RightBracket = b']',
    Caret = b'^',
    LeftBrace = b'{',
    VerticalBar = b'|',
    RightBrace = b'}',
    Tilde = b'~',
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Ident { raw: &'src str },
    Keyword(Keyword),
    Symbol(Symbol),
    String { raw: &'src str },
}
