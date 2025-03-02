use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    InvalidFormatting { line: String },
    NoObjectForValue { value: String },
    DoubleValueDefinition { object: String, value: String },
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ParseError::IOError(e) => e.fmt(f),
            Self::InvalidFormatting { line } => write!(f, "Invalid Formatting: {}", line),
            Self::DoubleValueDefinition { object, value } => {
                write!(f, "Double value definition of {} in {}", value, object)
            }
            Self::NoObjectForValue { value } => {
                write!(f, "Found value {} outside any object definition!", value)
            }
        }
    }
}
