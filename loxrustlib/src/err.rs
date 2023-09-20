use std::{error::Error, fmt, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
    internal: Option<Rc<LoxError>>,
}

impl LoxError {
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
            line: 0,
            internal: None,
        }
    }

    pub fn with_line(message: &str, line: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
            internal: None,
        }
    }

    pub fn with_message_line(message: String, line: usize) -> Self {
        Self {
            message,
            line,
            internal: None,
        }
    }

    pub fn with_internal(err: LoxError, line: usize) -> Self {
        Self {
            message: format!("- {}: {}\n", line, err),
            line,
            internal: Some(Rc::new(err)),
        }
    }
}

impl From<&dyn Error> for LoxError {
    fn from(err: &dyn Error) -> LoxError {
        LoxError {
            message: format!("- {}\n", err),
            line: 0,
            internal: None,
        }
    }
}

impl std::error::Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(internal_error) = &self.internal else {
            return writeln!(f, "Error occurred at line {}: {}", self.line, self.message)
        };

        write!(
            f,
            "Error occurred at line {}: {}\n{}",
            self.line, self.message, internal_error
        )
    }
}
