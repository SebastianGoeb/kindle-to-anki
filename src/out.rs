use std::error::Error;

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

pub fn write(words: &Vec<Word>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    for word in words {
        wtr.write_record(&StringRecord::from(word))?;
    }
    wtr.flush()?;
    Ok(())
}
