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
    use std::error;

    use crate::{cli, log_words};

    #[async_std::test]
    async fn should_log_words() -> Result<(), Box<dyn error::Error>> {
        let config = cli::Config {
            sqlite_uri: concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite").to_owned(),
            out_path: concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.test").to_owned(),
        };

        // when we log the words
        log_words(&config).await?;

        // shouldn't crash
        Ok(())
    }
}
