use std::error;

pub mod cli;
mod db;

pub async fn log_words(config: &cli::Config) -> Result<(), Box<dyn error::Error>> {
    println!("words from {}", config.path);

    let mut conn = db::connect(config.path.clone()).await?;
    let words = db::select_words(&mut conn).await?;

    for word in words {
        println!("{:?}", word);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error;

    use crate::{cli, log_words};

    #[async_std::test]
    async fn should_log_words() -> Result<(), Box<dyn error::Error>> {
        let config = cli::Config {
            path: concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite").to_owned(),
        };

        // when we log the words
        log_words(&config).await?;

        // shouldn't crash
        Ok(())
    }
}
