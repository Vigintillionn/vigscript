mod parser;
mod runtime;

fn main() {
    repl();
}

fn repl() {
    // Create a repl in the terminal
    println!("Repl version 0.1.0");

    loop {
        // Read a line
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Parse the line
        let tokens = parser::lexer::tokenize(&input);
        let mut parser = parser::Parser::new(&tokens);
        let ast = parser.produce_ast();

        // Evaluate the line
        let result = runtime::evaluate(ast);
        println!("{:?}", result);
    }
}
