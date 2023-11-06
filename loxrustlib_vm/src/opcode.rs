use std::fmt::Display;

pub enum OpCode {
    OpReturn,
    OpConstant { constant_location: usize },
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::OpReturn => format!("{:0>8} {}", u8::from(self), "OpReturn"),
                OpCode::OpConstant { constant_location } => format!(
                    "{:0>8} {} {:_>32}",
                    u8::from(self),
                    "OpConstant",
                    constant_location
                ),
                OpCode::OpNegate => format!("{:0>8} {}", u8::from(self), "OpNegate"),
                OpCode::OpAdd => format!("{:0>8} {}", u8::from(self), "OpAdd"),
                OpCode::OpSubtract => format!("{:0>8} {}", u8::from(self), "OpSubtract"),
                OpCode::OpMultiply => format!("{:0>8} {}", u8::from(self), "OpMultiply"),
                OpCode::OpDivide => format!("{:0>8} {}", u8::from(self), "OpDivide"),
            }
        )
    }
}

impl From<&OpCode> for u8 {
    fn from(val: &OpCode) -> Self {
        match val {
            OpCode::OpReturn => 0u8,
            OpCode::OpConstant {
                constant_location: _,
            } => 1u8,
            OpCode::OpNegate => 2u8,
            OpCode::OpAdd => 3u8,
            OpCode::OpSubtract => 4u8,
            OpCode::OpMultiply => 5u8,
            OpCode::OpDivide => 6u8,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        u8::from(&value)
    }
}
