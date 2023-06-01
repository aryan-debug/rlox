use crate::{parser::Parser, scanner::Scanner, interpreter::Interpreter};
use std::{
    fs,
    io::{self, Write},
    process,
};

enum Mode{
    File,
    Repl
}

pub struct CodeRunner {
    mode: Option<Mode>
}

impl CodeRunner {
    pub fn new() -> Self {
        CodeRunner { mode: None }
    }

    fn set_mode(&mut self, mode: Mode){
        self.mode = Some(mode);
    }

    pub fn run_file(&mut self, path: String) {
        self.set_mode(Mode::File);
        let content = fs::read_to_string(path).map_err(|err| {
            eprintln!("Error reading file: {err}");
            process::exit(1)
        });
        self.run(content.unwrap());
    }

    pub fn run_prompt(&mut self) {
        self.set_mode(Mode::Repl);
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Invalid input");
            if line.trim().is_empty() {
                break;
            };
            self.run(line);
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec());
        let statements = parser.parse();
        if !statements.is_empty() {
            let mut interpreter = Interpreter::new();
            interpreter.interpret(&statements);
        }
        else {
            self.handle_error();
        }
    }

    fn handle_error(&self){
        if let Mode::File = self.mode.as_ref().unwrap() {
            process::exit(65)
        }
    }
}
