use std::{error::Error, path::Path};

use csv::StringRecord;

use crate::db::Word;

impl From<&Word> for StringRecord {
    fn from(value: &Word) -> Self {
        csv::StringRecord::from(vec![
            value.id.to_owned(),
            value.word.to_owned(),
            value.stem.to_owned(),
            value.lang.to_owned(),
            value.category.to_string(),
            value.timestamp.to_string(),
            value.profileid.to_owned(),
        ])
    }
}

pub fn write<P: AsRef<Path>>(words: &Vec<Word>, path: P) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    for word in words {
        writer.write_record(&StringRecord::from(word))?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::db;

    macro_rules! some_word {
        () => {
            db::Word {
                id: "some_id".to_owned(),
                word: "some_word".to_owned(),
                stem: "some_stem".to_owned(),
                lang: "some_lang".to_owned(),
                category: 1,
                timestamp: 2,
                profileid: "some_profileid".to_owned(),
            }
        };
    }

    macro_rules! other_word {
        () => {
            db::Word {
                id: "other_id".to_owned(),
                word: "other_word".to_owned(),
                stem: "other_stem".to_owned(),
                lang: "other_lang".to_owned(),
                category: 3,
                timestamp: 4,
                profileid: "other_profileid".to_owned(),
            }
        };
    }

    #[test]
    fn should_nothing() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = tempfile::NamedTempFile::new().unwrap();
        crate::file::csv::write(&vec![], csvfile.path())?;
        assert_eq!(fs::read_to_string(csvfile.path())?, "");
        Ok(())
    }

    #[test]
    fn should_write_words() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = tempfile::NamedTempFile::new().unwrap();
        crate::file::csv::write(&vec![some_word!(), other_word!()], csvfile.path())?;
        assert_eq!(
            fs::read_to_string(csvfile.path())?,
            "some_id,some_word,some_stem,some_lang,1,2,some_profileid
other_id,other_word,other_stem,other_lang,3,4,other_profileid
"
        );
        Ok(())
    }
}
