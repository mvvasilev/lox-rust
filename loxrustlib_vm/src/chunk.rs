use crate::{opcode::OpCode, value::Value};

type OpCodeLine = (OpCode, u32);

pub struct Chunk {
    opcodes: Vec<OpCodeLine>,
    constants: Vec<Value>,

    last_line: u32,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            constants: Vec::new(),
            last_line: 0,
        }
    }

    pub fn push_opcode(&mut self, opcode: OpCode, line: u32) -> usize {
        self.opcodes.push((opcode, line));

        self.opcodes.len() - 1
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);

        self.constants.len() - 1
    }

    pub fn read_opcode_at(&self, location: usize) -> Option<&OpCodeLine> {
        self.opcodes.get(location)
    }

    pub fn read_constant_at(&self, location: usize) -> Option<&Value> {
        self.constants.get(location)
    }

    pub fn disassemble(&mut self) {
        println!("=== Disassembly ===");

        println!("=== Constants ===");

        for (i, constant) in self.constants.iter().enumerate() {
            println!("{:>4}: {}", i, constant);
        }

        println!("=== Opcodes ===");

        for (i, (opcode, line)) in self.opcodes.iter().enumerate() {
            self.disassemble_instruction(&i, opcode, line);

            self.last_line = *line;
        }
    }

    pub fn disassemble_instruction(&self, counter: &usize, opcode: &OpCode, line: &u32) {
        print!("{:_>4}: ", counter);

        if line == &self.last_line {
            print!("{:_>4} ", "|");
        } else {
            print!("{:_>4} ", line);
        }

        match opcode {
            OpCode::OpConstant { constant_location } => {
                print!("{} -> ", opcode);
                println!(
                    "{}",
                    self.read_constant_at(*constant_location)
                        .unwrap_or(&Value::Number(0.0f64))
                )
            }
            code => println!("{}", code),
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
