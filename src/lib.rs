use std::env;

use futures::TryStreamExt;
use sqlx::{Connection, SqliteConnection};

#[derive(sqlx::FromRow, Debug)]
struct Word {
    id: String,
    word: String,
    stem: String,
    lang: String,
    category: i32,
    timestamp: i32,
    profileid: String,
}

pub async fn log_words() -> Result<(), sqlx::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .to_str()
        .expect("parseable cwd")
        .to_string();
    let mut conn =
        SqliteConnection::connect(&format!("file:///{}/tmp/vocab.sqlite", cwd)[..]).await?;
    // let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    let mut rows = sqlx::query_as::<_, Word>("SELECT * FROM WORDS").fetch(&mut conn);

    while let Some(row) = rows.try_next().await? {
        println!("{:?}", &row);
    }

    Ok(())
}

// async fn select_words() -> sqlx::query::QueryAs<i64, Word, i64> {
//     sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
// }

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::{Connection, SqliteConnection};

    use crate::{log_words, Word};

    #[async_std::test]
    async fn should_get_many_rows() -> Result<(), sqlx::Error> {
        log_words().await?;
        Ok(())
    }

    #[async_std::test]
    #[ignore]
    async fn should_get_single_row() -> Result<(), sqlx::Error> {
        let cwd = env::current_dir()
            .unwrap()
            .to_str()
            .expect("parseable cwd")
            .to_string();
        let mut conn =
            SqliteConnection::connect(&format!("file:///{}/tmp/vocab.sqlite", cwd)[..]).await?;
        let row = sqlx::query_as::<_, Word>("SELECT * FROM WORDS")
            .fetch_all(&mut conn)
            .await?;
        println!("{:?}", dbg!(row));
        // let actual = select_words();
        // assert_eq!(1, 2);
        Ok(())
    }
}
