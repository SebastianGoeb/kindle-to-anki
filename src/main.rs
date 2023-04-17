use std::{env, error};

use kindle_to_anki::{cli, export_to_csv};

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let config = cli::Config::parse(&args).await?;
    export_to_csv(&config).await
}
