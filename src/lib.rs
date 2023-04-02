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

pub async fn get_words() -> Result<Vec<Word>, sqlx::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .to_str()
        .expect("parseable cwd")
        .to_string();
    let path: String = format!("file:///{}/tmp/vocab.sqlite", cwd);
    let mut conn = SqliteConnection::connect(&path).await?;
    sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
        .fetch_all(&mut conn)
        .await
}

#[cfg(test)]
mod tests {

    use crate::{get_words, Word};

    #[async_std::test]
    async fn should_get_words() -> Result<(), sqlx::Error> {
        let result = get_words().await?;
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
