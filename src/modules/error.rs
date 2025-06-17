use serde_json;
use std::io;

#[derive(Debug)] 
pub enum ParserError {
    IO(io::Error),
    JSON(serde_json::Error),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::IO(e) => write!(f, "Input/Output error: {}", e),
            ParserError::JSON(e) => write!(f, "JSON Parse/Stringify error: {}", e),
        }
    }
}
impl std::error::Error for ParserError {}
impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        ParserError::IO(err)
    }
}
impl From<serde_json::Error> for ParserError {
    fn from(err: serde_json::Error) -> Self {
        ParserError::JSON(err)
    }
}