use crate::{chunk::Chunk, opcode::OpCode, value::Value};

pub struct VM {
    chunk: Chunk,
    pc: usize,
    stack: Vec<Value>,

    enable_debug_tracing: bool,
}

macro_rules! binary_op {
    ($op:expr) => {
        {
            let Some(Value::Number(a)) = self.stack.pop() else {
                return VMInterpretResult::RuntimeError;
            };

            let Some(Value::Number(b)) = self.stack.pop() else {
                return VMInterpretResult::RuntimeError;
            };

            self.stack.push(Value::Number(a op b));
        }
    }
}

impl VM {


    pub fn new(enable_debug_tracing: bool) -> Self {
        Self {
            chunk: Chunk::new(),
            pc: 0,
            stack: Vec::new(),
            enable_debug_tracing,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> VMInterpretResult {
        self.chunk = chunk;
        self.pc = 0;

        self.run()
    }

    fn run(&mut self) -> VMInterpretResult {
        loop {
            let Some((instruction, line)) = self.chunk.read_opcode_at(self.pc) else { return VMInterpretResult::Ok; };

            if self.enable_debug_tracing {
                println!("{:?}", self.stack);
                self.chunk.disassemble_instruction(&self.pc, instruction, line);
            }

            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap_or(Value::Number(0.0)));

                    return VMInterpretResult::Ok;
                }
                OpCode::OpConstant { constant_location } => {
                    let Some(constant) = self.chunk.read_constant_at(*constant_location) else {
                        return VMInterpretResult::RuntimeError;
                    };

                    self.stack.push(constant.clone());

                    println!("{}", constant);
                }
                OpCode::OpNegate => {
                    let Some(Value::Number(value)) = self.stack.pop() else {
                        return VMInterpretResult::RuntimeError;
                    };

                    self.stack.push(Value::Number(-value));
                }
                OpCode::OpAdd => { self.arithmetic_op(|a, b| a + b); },
                OpCode::OpSubtract => { self.arithmetic_op(|a, b| a - b); },
                OpCode::OpMultiply => { self.arithmetic_op(|a, b| a * b); },
                OpCode::OpDivide => { self.arithmetic_op(|a, b| a / b); },
            }

            self.pc += 1;
        }
    }

    fn arithmetic_op<N>(&mut self, op: N) -> VMInterpretResult where N: Fn(f64, f64) -> f64 {
        let Some(Value::Number(b)) = self.stack.pop() else {
            return VMInterpretResult::RuntimeError;
        };

        let Some(Value::Number(a)) = self.stack.pop() else {
            return VMInterpretResult::RuntimeError;
        };

        self.stack.push(Value::Number(op(a, b)));

        VMInterpretResult::Ok
    }

    pub fn init(&mut self) {
        self.stack.clear();
    }

    pub fn free(&mut self) {}
}

impl Default for VM {
    fn default() -> Self {
        Self::new(false)
    }
}

pub enum VMInterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
