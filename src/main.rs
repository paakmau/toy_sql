use chumsky::{Parser, error::Simple, extra::Err, input::Input};
use parser::{lexer::lexer, parser::parser};

mod parser;

fn main() {
    let input = "SELECT * FROM t WHERE v = 'some_str'";
    let tokens = lexer::<Err<Simple<_, _>>>().parse(input);
    println!("{:?}", &tokens);

    let tokens = tokens.unwrap();

    let result = parser::<_, Err<Simple<_, _>>>().parse(
        tokens
            .as_slice()
            .map((0..input.len()).into(), |(t, s)| (t, s)),
    );
    println!("{:?}", result);
}
