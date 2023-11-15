use std::fs;
mod parser;
mod runtime;

fn main() {
    // check if there is a file to run in the arguments
    let args: Vec<String> = std::env::args().collect();
    let mut env = runtime::environment::Environment::new(None);
    env.declare_var("true".to_string(), runtime::values::RuntimeValue::Bool { value: true  }, true);
    env.declare_var("false".to_string(), runtime::values::RuntimeValue::Bool { value: false }, true);
    env.declare_var("null".to_string(), runtime::values::RuntimeValue::Null, true);

    if args.len() > 1 {
        let filename = &args[1];
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
    let tokens = parser::lexer::tokenize(&input);
    let mut parser = parser::parser::Parser::new(&tokens);
    let ast = parser.produce_ast();

    let result = runtime::interpreter::evaluate(ast, env);
    println!("{:?}", result);
}