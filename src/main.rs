use std::fs;
mod parser;

fn main() {
    // Read a file
    let src = fs::read_to_string("src/test.txt").unwrap();
    // Parse the file
    let tokens = parser::lexer::tokenize(&src);
    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.produce_ast();

    println!("{:?}", ast);
}
