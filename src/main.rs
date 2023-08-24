use std::{env, fs::read_to_string, io::Write};

use token::TokenKind;

mod err;
mod expr;
mod scan;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let scan = match read_to_string(&args[1]) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };

        let mut binding = scan.chars();
        let mut scanner = scan::Scanner::new(&mut binding);

        loop {
            match scanner.next() {
                Some(Ok(token)) if matches!(token.kind, TokenKind::EOF) => {
                    println!("{:#?}", token);
                    break;
                }
                Some(Ok(token)) => println!("{:#?}", token),
                Some(Err(e)) => println!("{:#?}", e),
                None => break,
            }
        }
    } else {
        loop {
            let input = prompt("> ");

            if input.is_empty() {
                break;
            };

            let mut binding = input.chars();
            let mut scanner = scan::Scanner::new(&mut binding);

            loop {
                match scanner.next() {
                    Some(Ok(token)) if matches!(token.kind, TokenKind::EOF) => {
                        println!("{:#?}", token);
                        break;
                    }
                    Some(Ok(token)) => println!("{:#?}", token),
                    Some(Err(e)) => println!("{:#?}", e),
                    None => break,
                }
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
