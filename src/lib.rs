use std::error;

pub mod cli;
mod db;
mod file;

pub async fn log_words(config: &cli::Config) -> Result<(), Box<dyn error::Error>> {
    println!("words from {}", config.sqlite_uri);

    let mut conn = db::connect(config.sqlite_uri.clone()).await?;
    let words = db::select_words(&mut conn).await?;

    file::csv::write(&words, &config.out_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error, fs};

    use crate::{cli, log_words};

    #[async_std::test]
    async fn should_log_words() -> Result<(), Box<dyn error::Error>> {
        let csvfile = tempfile::NamedTempFile::new()?;
        let config = cli::Config {
            sqlite_uri: concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite").to_owned(),
            out_path: csvfile.path().to_owned(),
        };

        // when we log the words
        log_words(&config).await?;

        // should output csv
        let contents = fs::read_to_string(csvfile.path())?;
        assert_eq!(
            contents.lines().next(),
            Some("de:verwendet,verwendet,verwenden,de,0,1643718511733,")
        );
        assert_eq!(contents.lines().count(), 121);
        Ok(())
    }
}
