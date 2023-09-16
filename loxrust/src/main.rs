use std::{env, fs::read_to_string, io::Write};

use loxrustlib::{expr, parser, printer, scan};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let scan = match read_to_string(&args[1]) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };

        let scanner = scan::Scanner::new(&scan);
        let mut parser = parser::Parser::new(scanner);

        match parser.parse() {
            Some(tree) => {
                let printer = printer::PrettyPrinter::new();

                printer.print(&tree.into())
            }
            None => println!("Failed to parse: Invalid syntax"),
        }
    } else {
        let mut buffer: Vec<String> = Vec::new();

        loop {
            let input = prompt("> ");

            if input.is_empty() {
                break;
            };

            buffer.push(input.clone());

            let scanner = scan::Scanner::new(&input);
            let mut parser = parser::Parser::new(scanner);

            match parser.parse() {
                Some(tree) => {
                    let printer = printer::PrettyPrinter::new();

                    printer.print(&tree.into())
                }
                None => println!("Failed to parse '{}': Invalid syntax", input),
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
