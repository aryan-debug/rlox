use std::{fs, process, io::{self, Write}};

use crate::scanner::Scanner;

pub struct CodeRunner{
    had_error: bool
}

impl CodeRunner{

    pub fn new() -> Self{
        CodeRunner{had_error: false}
    }

    pub fn run_file(&mut self, path: String) {
        let content = fs::read_to_string(path).map_err(|err| {
            eprintln!("Error reading file: {err}");
            process::exit(1)
        });
        self.run(content.unwrap());
        
    }
    
    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Invalid input");
            if line.trim().is_empty() {
                break;
            };
            self.run(line);
            self.had_error = false;
        }
    }
    
    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();
        
        for token in tokens {
            println!("{:?}", token);
        }
       
    }
    
    pub fn error(&mut self, line: usize, message: &str){
        self.report(line, String::new(), message);
    }
    
    fn report(&mut self, line: usize, location: String, message: &str){
        println!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }
    
}