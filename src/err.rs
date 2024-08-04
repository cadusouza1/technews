use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    SelectorParseError(String),
    MissingAttribute(String),
    DateParseError(String),
    TimeParseError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SelectorParseError(err) => write!(f, "Selector parse error: {}", err),
            ParseError::MissingAttribute(err) => write!(f, "Missing attribute: {}", err),
            ParseError::DateParseError(err) => write!(f, "Date parse error: {}", err),
            ParseError::TimeParseError(err) => write!(f, "Time parse error: {}", err),
        }
    }
}

impl Error for ParseError {}
