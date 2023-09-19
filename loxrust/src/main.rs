use std::{
    env,
    fs::read_to_string,
    io::{Bytes, Write},
};

use loxrustlib::{interpreter, parser, scan};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut interpreter = interpreter::Interpreter::new();

    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let scan = match read_to_string(&args[1]) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };

        let scanner = scan::Scanner::new(&scan);
        let mut parser = parser::Parser::new(scanner);

        let statements = match parser.parse() {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to parse: {}", e);
                return;
            }
        };

        match interpreter.interpret(statements) {
            Ok(_) => (),
            Err(e) => println!("Failed to execute: {}", e),
        }
    } else {
        loop {
            let input = prompt("> ");

            if input.is_empty() {
                break;
            };

            let scanner = scan::Scanner::new(&input);
            let mut parser = parser::Parser::new(scanner);

            let statements = match parser.parse() {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to parse: {}", e);
                    continue;
                }
            };

            match interpreter.interpret(statements) {
                Ok(_) => (),
                Err(e) => println!("Failed to execute: {}", e),
            }
        }
    }
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}
