use kindle_to_anki::get_words;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let words = get_words().await?;
    for word in words {
        println!("{:?}", word);
    }
    Ok(())
}
