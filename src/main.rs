use chumsky::{Parser, error::Simple, extra::Err};
use parser::lexer::lexer;

mod parser;

fn main() {
    let input = "SELECT * FROM t WHERE v = 'some_str'";
    let result = lexer::<Err<Simple<_, _>>>().parse(input);
    println!("{:?}", result);
}
