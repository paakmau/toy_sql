use chumsky::{Parser, error::Simple, extra::Err, input::Input};
use parser::{lexer::lexer, parser::tree};

mod parser;

fn main() {
    let input = "SELECT * FROM t WHERE v = 'some_str'";
    let input = "SELECT abccc";
    let tokens = lexer::<Err<Simple<_, _>>>().parse(input);
    println!("{:?}", &tokens);

    let tokens = tokens.unwrap();

    let result = tree::<_, Err<Simple<_, _>>>().parse(
        tokens
            .as_slice()
            .map((0..input.len()).into(), |(t, s)| (t, s)),
    );
    println!("{:?}", result);
}
