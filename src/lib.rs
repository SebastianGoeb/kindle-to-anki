use std::error;

pub mod cli;
mod db;
mod file;
mod model;

pub async fn export_to_csv(config: &cli::Config) -> Result<(), Box<dyn error::Error>> {
    eprintln!("importing from {}", config.sqlite_uri);

    let pool = db::connect(config.sqlite_uri.clone()).await?;
    let notes = import_notes(&pool).await?;

    eprintln!("exporting to {}", config.out_path.to_string_lossy());
    file::csv::write(&notes, &config.out_path)?;
    Ok(())
}

async fn import_notes(pool: &sqlx::SqlitePool) -> Result<Vec<model::Note>, sqlx::Error> {
    let words = db::select_words(&pool).await?;
    let it = words
        .into_iter()
        .map(|word| async move { word_to_note(word, pool).await });
    let notes: Vec<Result<model::Note, sqlx::Error>> = futures::future::join_all(it).await;
    let oks: Result<Vec<model::Note>, sqlx::Error> = notes.into_iter().collect();
    return oks;
}

async fn word_to_note(word: db::Word, pool: &sqlx::SqlitePool) -> Result<model::Note, sqlx::Error> {
    let lookups: Vec<db::Lookup> = db::find_lookups_by_word(&word.id, pool).await?;
    let usages = lookups.into_iter().map(|lookup| lookup.usage).collect();
    Ok(model::Note::new(word, usages))
}

#[cfg(test)]
mod tests {
    use std::{error, fs};

    use crate::{cli, export_to_csv};

    #[async_std::test]
    async fn should_export_words_and_usage_to_csv() -> Result<(), Box<dyn error::Error>> {
        let csvfile = tempfile::NamedTempFile::new()?;
        let config = cli::Config {
            sqlite_uri: concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite").to_owned(),
            out_path: csvfile.path().to_owned(),
        };

        // when we log the words
        export_to_csv(&config).await?;

        // should output csv
        let contents = fs::read_to_string(csvfile.path())?;
        assert_eq!(
            contents.lines().next(),
            Some("de:verwendet,verwendet,verwenden,de,\"Für den beliebten Käsekuchen verwendet man Speisequark, der dicker als Joghurt ist und nicht so säuerlich. \"")
        );
        assert_eq!(contents.lines().count(), 122); // text lines, not csv rows
        Ok(())
    }
}
