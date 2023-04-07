use core::fmt;
use std::{error, fs, path::PathBuf};

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
    pub sqlite_uri: String,
    pub out_path: PathBuf,
}

impl Config {
    pub async fn parse(args: &[String]) -> Result<Self, CliError> {
        match args {
            [in_path, out_path] => {
                let sqlite_uri =
                    format!("file:///{}", fs::canonicalize(in_path)?.to_string_lossy());
                Ok(Config {
                    sqlite_uri,
                    out_path: out_path.into(),
                })
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
        let config = Config::parse(&vec![
            "./test/vocab.sqlite".to_owned(),
            "./test/vocab.csv".to_owned(),
        ])
        .await?;
        assert_eq!(
            config,
            Config {
                sqlite_uri: concat!("file:///", env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite")
                    .to_owned(),
                out_path: "./test/vocab.csv".into()
            }
        );
        Ok(())
    }
}
