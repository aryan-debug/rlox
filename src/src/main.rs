use code_runner::CodeRunner;
use std::env;
mod code_runner;
mod expr;
mod interpreter;
mod keywords;
mod literal;
mod parser;
mod scanner;
mod token;
mod token_type;
mod error;
mod stmt;
mod environment;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut code_runner = CodeRunner::new();

    if args.len() >= 3 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        code_runner.run_file(args[1].to_owned());
    } else {
        code_runner.run_prompt();
    }
}
