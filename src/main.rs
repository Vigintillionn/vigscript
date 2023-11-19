use std::fs;
mod frontend;
mod runtime;

fn main() {
    // check if there is a file to run in the arguments
    let args: Vec<String> = std::env::args().collect();
    let mut env = runtime::environment::create_global_environment();

    if args.len() > 1 {
        let filename = &args[1];
        if !filename.ends_with(".vig") {
            panic!("File must end with .vig");
        }
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        interpret(&contents, &mut env);
    } else {
        repl(&mut env);
    }
}

fn repl(env: &mut runtime::environment::Environment) {
    // Create a repl in the terminal
    println!("Repl version 0.1.0");
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" {
            break;
        }
        interpret(&input, env);
        input.clear();
    }
}

fn interpret(input: &str, env: &mut runtime::environment::Environment) {
    let tokens = frontend::lexer::tokenize(&input);
    print!("{:?}", tokens);
    let mut parser = frontend::parser::Parser::new(&tokens);
    let ast = parser.produce_ast();
    println!("{:?}", ast);

    runtime::interpreter::evaluate(ast, env);
    //let result = runtime::interpreter::evaluate(ast, env);
    //println!("{:?}", result);
}