use std::{io::Write, env};

// use std::{
//     env,
//     fs::read_to_string,
//     io::Write,
// };
use loxrustlib_vm::{chunk::Chunk, opcode::OpCode, value::Value, vm::VM};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        print!("Usage: rlox [script]");
        
        return;
    }

    if args.len() == 2 {
        
    }

    // let mut vm: VM = VM::new(true);

    // vm.init();

    // let mut chunk: Chunk = Chunk::new();

    // let loc1 = chunk.push_constant(Value::Number(1.0));
    // let loc3 = chunk.push_constant(Value::Number(3.0));
    // let loc5 = chunk.push_constant(Value::Number(5.0));

    // chunk.push_opcode(OpCode::OpConstant { constant_location: loc3 }, 1);

    // chunk.push_opcode(OpCode::OpConstant { constant_location: loc1 }, 1);

    // chunk.push_opcode(OpCode::OpAdd, 1);

    // chunk.push_opcode(OpCode::OpConstant { constant_location: loc5 }, 1);

    // chunk.push_opcode(OpCode::OpDivide, 1);

    // chunk.push_opcode(OpCode::OpNegate, 1);

    // chunk.push_opcode(OpCode::OpReturn, 1);

    //chunk.disassemble();

    // vm.interpret(chunk);

    // vm.free();
    // let args: Vec<String> = env::args().collect();

    // let mut interpreter = interpreter::Interpreter::new();

    // if args.len() > 2 {
    //     print!("Usage: rlox [script]");
    // } else if args.len() == 2 {
    //     let scan = match read_to_string(&args[1]) {
    //         Ok(a) => a,
    //         Err(e) => panic!("{}", e),
    //     };

    //     let scanner = scan::Scanner::new(&scan);
    //     let mut parser = parser::Parser::new(scanner);

    //     let statements = match parser.parse() {
    //         Ok(s) => s,
    //         Err(e) => {
    //             println!("Failed to parse: {}", e);
    //             return;
    //         }
    //     };

    //     println!("{:?}", statements);

    //     match interpreter.interpret(statements) {
    //         Ok(_) => (),
    //         Err(e) => println!("Failed to execute: {}", e),
    //     }
    // } else {
    //     loop {
    //         let input = prompt("> ");

    //         if input.is_empty() {
    //             break;
    //         };

    //         let scanner = scan::Scanner::new(&input);
    //         let mut parser = parser::Parser::new(scanner);

    //         let statements = match parser.parse() {
    //             Ok(s) => s,
    //             Err(e) => {
    //                 println!("Failed to parse: {}", e);
    //                 continue;
    //             }
    //         };

    //         println!("{:?}", statements);

    //         match interpreter.interpret(statements) {
    //             Ok(_) => (),
    //             Err(e) => println!("Failed to execute: {}", e),
    //         }
    //     }
    // }
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
