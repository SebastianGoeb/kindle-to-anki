use sqlx::{Connection, SqliteConnection};

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct Word {
    pub id: String,
    pub word: String,
    pub stem: String,
    pub lang: String,
    pub category: i32,
    pub timestamp: i64,
    pub profileid: String,
}

pub async fn connect(path: String) -> Result<SqliteConnection, sqlx::Error> {
    SqliteConnection::connect(&path).await
}

pub async fn select_words(conn: &mut SqliteConnection) -> Result<Vec<Word>, sqlx::Error> {
    sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
        .fetch_all(conn)
        .await
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::{connect, select_words, Word};

    #[async_std::test]
    async fn should_select_words() -> Result<(), sqlx::Error> {
        // given a test file with some words
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite");

        // when we select the words from the test file
        let mut conn = connect(path.to_owned()).await?;
        let result = select_words(&mut conn).await?;

        // should return the words
        assert_eq!(result.len(), 121);
        assert_eq!(
            result[0],
            Word {
                id: String::from("de:verwendet"),
                word: String::from("verwendet"),
                stem: String::from("verwenden"),
                lang: String::from("de"),
                category: 0,
                timestamp: 1643718511733,
                profileid: String::from(""),
            }
        );
        Ok(())
    }
}
