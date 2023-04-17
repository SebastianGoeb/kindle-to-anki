use crate::db;

pub struct Note {
    pub id: String,
    pub word: String,
    pub stem: String,
    pub lang: String,
    pub usages: Vec<String>,
}

impl Note {
    pub fn new(word: db::Word, usages: Vec<String>) -> Self {
        Note {
            id: word.id,
            word: word.word,
            stem: word.stem,
            lang: word.lang,
            usages,
        }
    }
}
