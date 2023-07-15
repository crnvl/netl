use logic::exec::interpret;

use crate::logic::ast::parse;

mod logic;

fn main() {
    let script = std::fs::read_to_string("./examples/test.nl").unwrap();

    let tokens = logic::tokenizer::tokenize(&script);

    println!("{:?}", tokens);

    let result = parse(tokens);

    println!("{:?}", result);

    interpret(result.unwrap());
}
