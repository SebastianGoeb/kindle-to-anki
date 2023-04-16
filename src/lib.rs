use std::error;

use futures::{self, StreamExt};

pub mod cli;
mod db;
mod file;
mod model;

pub async fn export_to_csv(config: &cli::Config) -> Result<(), Box<dyn error::Error>> {
    eprintln!("importing from {}", config.sqlite_uri);

    let conn = db::connect(config.sqlite_uri.clone()).await?;
    let notes = import_notes(conn).await?;

    eprintln!("exporting to {}", config.out_path.to_string_lossy());
    file::csv::write(&notes, &config.out_path)?;
    Ok(())
}

async fn import_notes(
    mut conn: sqlx::SqliteConnection,
) -> Result<Vec<model::Note>, Box<dyn error::Error>> {
    let it = db::select_words(&mut conn)
        .await?
        .iter()
        .map(|word| async { word_to_note(&word, conn).await });
    // let st = futures::stream::iter(it).buffer_unordered(10);
    let notes: Vec<model::Note> = futures::future::join_all(it)
        .await
        .iter()
        .filter_map(|res| res.ok())
        .collect();
    Ok(notes)
}

async fn word_to_note(
    word: &db::Word,
    mut conn: sqlx::SqliteConnection,
) -> Result<model::Note, Box<dyn error::Error>> {
    let lookups: Vec<db::Lookup> = db::find_lookups_by_word(&word.id, &mut conn).await?;
    let usages = lookups.into_iter().map(|lookup| lookup.usage).collect();
    Ok(model::Note::new(&word, usages))
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
            Some("de:verwendet,verwendet,verwenden,de,")
        );
        assert_eq!(contents.lines().count(), 121);
        Ok(())
    }
}
