use kindle_to_anki::log_words;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    log_words().await
}
