use std::{env, fs::read_to_string, io::Write};

use expr::PrettyPrinter;
use token::TokenKind;

mod err;
mod expr;
mod parser;
mod scan;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    PrettyPrinter::new().print(&mut expr::Binary {
        left: Box::new(expr::Unary {
            operator: token::Token {
                kind: TokenKind::MINUS,
                line: 0,
            },
            right: Box::new(expr::Literal {
                literal: token::Token {
                    kind: TokenKind::Number(123.0),
                    line: 0,
                },
            }),
        }),
        operator: token::Token {
            kind: TokenKind::STAR,
            line: 0,
        },
        right: Box::new(expr::Grouping {
            expression: Box::new(expr::Literal {
                literal: token::Token {
                    kind: TokenKind::Number(45.67),
                    line: 0,
                },
            }),
        }),
    });

    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {                                             
        let scan = match read_to_string(&args[1]) {                         
            Ok(a) => a,                                                     
            Err(e) => panic!("{}", e),                                      
        };                                                                  
                                        
        let mut scanner = scan::Scanner::new(scan);                                                                                                                           
                                                             
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
            
            let mut scanner = scan::Scanner::new(input);

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
