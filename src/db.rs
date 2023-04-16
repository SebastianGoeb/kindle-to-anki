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

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct Lookup {
    pub id: String,
    pub word_key: String,
    pub book_key: String,
    pub dict_key: String,
    pub pos: String,
    pub usage: String,
    pub timestamp: i64,
}

pub async fn connect(path: String) -> Result<SqliteConnection, sqlx::Error> {
    SqliteConnection::connect(&path).await
}

pub async fn select_words(conn: &mut SqliteConnection) -> Result<Vec<Word>, sqlx::Error> {
    sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
        .fetch_all(conn)
        .await
}

pub async fn find_lookups_by_word(
    word_key: &str,
    conn: &mut SqliteConnection,
) -> Result<Vec<Lookup>, sqlx::Error> {
    sqlx::query_as::<_, Lookup>("SELECT * FROM LOOKUPS WHERE word_key = ?")
        .bind(word_key)
        .fetch_all(conn)
        .await
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

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

    #[async_std::test]
    async fn should_find_multiple_lookups_for_word() -> Result<(), sqlx::Error> {
        // given a test file with some words
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite");

        // when we select the words from the test file
        let mut conn = connect(path.to_owned()).await?;
        let result = find_lookups_by_word("en:validate", &mut conn).await?;

        // should return the words
        let usage_1 = "They validate other people\u{2019}s feelings. ";
        let usage_2 = "If you first validate their stance (“That\u{2019}s interesting, I never thought of it that way…”) and then present your own opinion (“Something I recently learned is this…”) and then let them know that they still hold their own power in the conversation by asking their opinion (“What do you think about that?”), you open them up to engaging in a conversation where both of you can learn rather than just defend. ";
        assert_eq!(
            result,
            vec![
                Lookup {
                    id: "CR!0J9MNQDWAN3VS0RM5CJ7NDKDPMTF:EA3AE709:52112:11".to_owned(),
                    word_key: "en:validate".to_owned(),
                    book_key: "CR!0J9MNQDWAN3VS0RM5CJ7NDKDPMTF:EA3AE709".to_owned(),
                    dict_key: "B00771V9HS".to_owned(),
                    pos: "52112".to_owned(),
                    usage: usage_1.to_owned(),
                    timestamp: 1679510148955
                },
                Lookup {
                    id: "CR!0J9MNQDWAN3VS0RM5CJ7NDKDPMTF:EA3AE709:51616:11".to_owned(),
                    word_key: "en:validate".to_owned(),
                    book_key: "CR!0J9MNQDWAN3VS0RM5CJ7NDKDPMTF:EA3AE709".to_owned(),
                    dict_key: "B00771V9HS".to_owned(),
                    pos: "51616".to_owned(),
                    usage: usage_2.to_owned(),
                    timestamp: 1679510013059
                }
            ]
        );
        Ok(())
    }

    #[async_std::test]
    async fn should_find_no_lookups_for_nonexistent_word() -> Result<(), sqlx::Error> {
        // given a test file with some words
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test/vocab.sqlite");

        // when we select the words from the test file
        let mut conn = connect(path.to_owned()).await?;
        let result = find_lookups_by_word("doesn't_exist", &mut conn).await?;

        // should return the words
        assert_eq!(result, vec![]);
        Ok(())
    }
}
