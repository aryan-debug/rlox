use std::env;
use code_runner::CodeRunner;
mod expr;
mod scanner;
mod token_type;
mod token;
mod literal;
mod code_runner;
mod keywords;


fn main() {
    
    let args = env::args().collect::<Vec<String>>();
    let mut code_runner = CodeRunner::new();
    if args.len() > 3 {
        println!("Usage: rlox [script]");
    } else if args.len() == 3 {
        code_runner.run_file(args[2].to_owned());
    } else {
        code_runner.run_prompt();
    }
}

