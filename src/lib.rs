mod cli;
mod db;

pub async fn log_words() -> Result<(), sqlx::Error> {
    let path = cli::get_path();
    let mut conn = db::connect(path).await?;
    let words = db::select_words(&mut conn).await?;

    for word in words {
        println!("{:?}", word);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::log_words;

    #[async_std::test]
    async fn should_log_words() -> Result<(), sqlx::Error> {
        // when we log the words
        log_words().await?;
        Ok(())
    }
}
