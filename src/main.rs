use std::fs;
mod parser;
mod runtime;

fn main() {
    // Read a file
    let src = fs::read_to_string("src/test.txt").unwrap();
    // Parse the file
    let tokens = parser::lexer::tokenize(&src);
    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.produce_ast();

    let result = runtime::evaluate(ast);
    println!("{:?}", result);
}
