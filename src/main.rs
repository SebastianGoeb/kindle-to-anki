use kindle_to_anki::ingest;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let words = ingest().await?;
    for word in words {
        println!("{:?}", word);
    }
    Ok(())
}
