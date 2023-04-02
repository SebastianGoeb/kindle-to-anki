use std::{env, error};

use kindle_to_anki::{cli, log_words};

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let config = cli::Config::parse(&args).await?;
    log_words(&config).await
}
