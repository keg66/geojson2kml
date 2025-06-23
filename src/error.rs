use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GeojsonError {
    IoError(io::Error),
    JsonParseError(serde_json::Error),
    InvalidInput(String),
    NoMatchFound(String),
    UserCancelled,
}

impl fmt::Display for GeojsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeojsonError::IoError(e) => write!(f, "File I/O error: {}", e),
            GeojsonError::JsonParseError(e) => write!(f, "JSON parsing error: {}", e),
            GeojsonError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GeojsonError::NoMatchFound(query) => write!(f, "No matches found for: {}", query),
            GeojsonError::UserCancelled => write!(f, "Operation cancelled by user"),
        }
    }
}

impl std::error::Error for GeojsonError {}

impl From<io::Error> for GeojsonError {
    fn from(error: io::Error) -> Self {
        GeojsonError::IoError(error)
    }
}

impl From<serde_json::Error> for GeojsonError {
    fn from(error: serde_json::Error) -> Self {
        GeojsonError::JsonParseError(error)
    }
}

pub type Result<T> = std::result::Result<T, GeojsonError>;