use core::fmt;
use std::error;

use async_std::{fs, path::PathBuf};

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

impl Config {
    pub async fn parse(args: &[String]) -> Result<Self, CliError> {
        match args {
            [path] => {
                let sqlite_uri = format!(
                    "file:///{}",
                    fs::canonicalize(PathBuf::from(path))
                        .await?
                        .to_string_lossy()
                );
                Ok(Config { path: sqlite_uri })
            }
            _ => Err(CliError::Args),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::Config;

    use super::CliError;

    #[async_std::test]
    async fn should_parse_config() -> Result<(), CliError> {
        let config = Config::parse(&vec!["./test/vocab.sqlite".to_owned()]).await?;
        assert_eq!(
            config,
            Config {
                path: concat!("file:///", env!("CARGO_MANIFEST_DIR"), "/test/vocab.db").to_owned()
            }
        );
        Ok(())
    }
}
