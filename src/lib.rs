use std::env;

use sqlx::{Connection, SqliteConnection};

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct Word {
    pub id: String,
    pub word: String,
    pub stem: String,
    pub lang: String,
    pub category: i32,
    pub timestamp: i32,
    pub profileid: String,
}

pub async fn ingest() -> Result<Vec<Word>, sqlx::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .to_str()
        .expect("parseable cwd")
        .to_string();
    let path: String = format!("file:///{}/tmp/vocab.sqlite", cwd);
    let mut conn = connect(path).await?;
    select_words().fetch_all(&mut conn).await
}

async fn connect(path: String) -> Result<SqliteConnection, sqlx::Error> {
    SqliteConnection::connect(&path).await
}

fn select_words(
) -> sqlx::query::QueryAs<'static, sqlx::Sqlite, Word, sqlx::sqlite::SqliteArguments<'static>> {
    sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::{connect, select_words, Word};

    #[async_std::test]
    async fn should_get_words() -> Result<(), sqlx::Error> {
        // given a test file with some words
        let cwd = env::current_dir()
            .unwrap()
            .to_str()
            .expect("parseable cwd")
            .to_string();
        let path: String = format!("file:///{}/tmp/vocab.sqlite", cwd);

        // when we select the words from the test file
        let mut conn = connect(path).await?;
        let result = select_words().fetch_all(&mut conn).await?;

        // should result the words
        assert_eq!(result.len(), 121);
        assert_eq!(
            result[0],
            Word {
                id: String::from("de:verwendet"),
                word: String::from("verwendet"),
                stem: String::from("verwenden"),
                lang: String::from("de"),
                category: 0,
                timestamp: -1253962635,
                profileid: String::from(""),
            }
        );
        Ok(())
    }
}
