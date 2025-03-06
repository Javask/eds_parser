use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    ParseIntError {
        object: String,
        section: String,
        err: std::num::ParseIntError,
    },
    ParseHexError {
        object: String,
        section: String,
        value: String,
    },
    ParseOctalError {
        object: String,
        section: String,
        value: String,
    },
    ParseFloatError {
        object: String,
        section: String,
        err: std::num::ParseFloatError,
    },
    InvalidFormatting {
        line: String,
    },
    NoObjectForValue {
        value: String,
    },
    DoubleValueDefinition {
        object: String,
        section: String,
    },
    DoubleSectionDefinition {
        section: String,
    },
    MissingRequiredObject {
        object: String,
        section: String,
    },
    MissingRequiredSection {
        section: String,
    },
    InvalidValueFormat {
        object: String,
        section: String,
    },
    ParseTimeError {
        date_obj: String,
        time_obj: String,
        section: String,
    },
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::ParseHexError {
                object,
                section,
                value,
            } => {
                write!(
                    f,
                    "Failed to parse hex value \"{}\" for \"{}\" in section \"{}\"",
                    value, object, section
                )
            }
            Self::DoubleSectionDefinition { section } => {
                write!(f, "Double definition of section \"{}\"", section)
            }
            Self::ParseOctalError {
                object,
                section,
                value,
            } => {
                write!(
                    f,
                    "Failed to parse octal value \"{}\" for \"{}\" in section \"{}\"",
                    value, object, section
                )
            }
            ParseError::IOError(e) => e.fmt(f),
            Self::ParseFloatError {
                object,
                section,
                err,
            } => write!(
                f,
                "Error parsing float value \"{}\" in section \"{}\": {}",
                object, section, err
            ),
            Self::ParseIntError {
                object,
                section,
                err,
            } => write!(
                f,
                "Error parsing int value \"{}\" in section \"{}\": {}",
                object, section, err
            ),
            Self::InvalidFormatting { line } => write!(f, "Invalid Formatting: {}", line),
            Self::DoubleValueDefinition { object, section } => {
                write!(
                    f,
                    "Double value definition of \"{}\" in \"{}\"",
                    object, section
                )
            }
            Self::NoObjectForValue { value } => {
                write!(
                    f,
                    "Found value \"{}\" outside any object definition!",
                    value
                )
            }
            Self::MissingRequiredObject { object, section } => {
                write!(
                    f,
                    "Missing object \"{}\" in section \"{}\"",
                    object, section
                )
            }
            Self::MissingRequiredSection { section } => {
                write!(f, "Missing section \"{}\"", section)
            }
            Self::InvalidValueFormat { object, section } => {
                write!(
                    f,
                    "Invalid formatting in value \"{}\" in section \"{}\"",
                    object, section
                )
            }
            Self::ParseTimeError {
                date_obj,
                time_obj,
                section,
            } => {
                write!(
                    f,
                    "Invalid time specified in \"{}\" and \"{}\" in section \"{}\"",
                    date_obj, time_obj, section
                )
            }
        }
    }
}
