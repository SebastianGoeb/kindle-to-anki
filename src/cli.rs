use core::fmt;
use std::error;

#[derive(Debug)]
pub enum CliError {
    Args,
    Path(String),
    IO(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Args => write!(f, "usage: kindle-to-anki /path/to/vocab.db"),
            CliError::Path(path) => write!(f, "invalid path: {}", path),
            CliError::IO(err) => write!(f, "IO error: {:?}", err),
        }
    }
}

impl error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
        CliError::IO(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub path: String,
}
