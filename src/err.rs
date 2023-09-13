use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
            line: 0,
        }
    }

    pub fn with_line(message: String, line: usize) -> Self {
        Self { message, line }
    }

    pub fn with_internal(err: LoxError, line: usize) -> Self {
        Self {
            message: format!("- {}: {}\n", err.line, err.message),
            line,
        }
    }
}

impl From<&dyn Error> for LoxError {
    fn from(err: &dyn Error) -> LoxError {
        LoxError {
            message: format!("- {}\n", err),
            line: 0,
        }
    }
}

impl std::error::Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred at line {}: {}", self.line, self.message)
    }
}
