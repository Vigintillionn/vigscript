use std::fs;
mod parser;
mod runtime;

fn main() {
    // check if there is a file to run in the arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        println!("{}", &filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        interpret(&contents);
    } else {
        repl();
    }
}

fn repl() {
    // Create a repl in the terminal
    println!("Repl version 0.1.0");
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" {
            break;
        }
        interpret(&input);
        input.clear();
    }
}

fn interpret(input: &str) {
    let tokens = parser::lexer::tokenize(&input);
    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.produce_ast();

    let result = runtime::evaluate(ast);
    println!("{:?}", result);
}