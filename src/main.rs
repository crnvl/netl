use logic::exec::interpret;

use crate::logic::ast::parse;

mod logic;

fn main() {
/*     let script = std::fs::read_to_string("./examples/test.nl").unwrap();

    let script = script.lines().filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n");
    
    let tokens = tokenize(&script);

    let result = parse(tokens);
    
    interpreter::interpret(result.unwrap()); */

    let script = std::fs::read_to_string("./examples/test.nl").unwrap();

    let tokens = logic::tokenizer::tokenize(&script);

    let result = parse(tokens);

    interpret(result.unwrap());
}
