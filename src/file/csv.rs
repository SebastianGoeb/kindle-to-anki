use crate::model;
use csv::StringRecord;
use std::{error::Error, path::Path};

impl From<&model::Note> for StringRecord {
    fn from(value: &model::Note) -> Self {
        csv::StringRecord::from(vec![
            value.id.to_owned(),
            value.word.to_owned(),
            value.stem.to_owned(),
            value.lang.to_owned(),
            value
                .usages
                .iter()
                .map(|usage| format!("<p>\n{}\n</p>", usage))
                .intersperse("\n".to_owned())
                .collect(),
        ])
    }
}

pub fn write<P: AsRef<Path>>(words: &Vec<model::Note>, path: P) -> Result<(), Box<dyn Error>> {
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

    use super::*;

    const EMPTY_NOTE: model::Note = model::Note {
        id: String::new(),
        word: String::new(),
        stem: String::new(),
        lang: String::new(),
        usages: vec![],
    };

    macro_rules! some_note {
        () => {
            model::Note {
                id: "some_id".to_owned(),
                word: "some_word".to_owned(),
                stem: "some_stem".to_owned(),
                lang: "some_lang".to_owned(),
                usages: vec!["some_usage".to_owned()],
            }
        };
    }

    macro_rules! other_note {
        () => {
            model::Note {
                id: "other_id".to_owned(),
                word: "other_word".to_owned(),
                stem: "other_stem".to_owned(),
                lang: "other_lang".to_owned(),
                usages: vec!["other_usage".to_owned()],
            }
        };
    }

    #[test]
    fn should_write_nothing() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = tempfile::NamedTempFile::new().unwrap();
        crate::file::csv::write(&vec![], csvfile.path())?;
        assert_eq!(fs::read_to_string(csvfile.path())?, "");
        Ok(())
    }

    #[test]
    fn should_write_empty_note() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = tempfile::NamedTempFile::new().unwrap();
        crate::file::csv::write(&vec![EMPTY_NOTE], csvfile.path())?;
        assert_eq!(fs::read_to_string(csvfile.path())?, ",,,,\n");
        Ok(())
    }

    #[test]
    fn should_write_multiple_notes() -> Result<(), Box<dyn std::error::Error>> {
        let csvfile = tempfile::NamedTempFile::new().unwrap();
        crate::file::csv::write(&vec![some_note!(), other_note!()], csvfile.path())?;
        assert_eq!(
            fs::read_to_string(csvfile.path())?,
            "some_id,some_word,some_stem,some_lang,\"<p>
some_usage
</p>\"
other_id,other_word,other_stem,other_lang,\"<p>
other_usage
</p>\"
"
        );
        Ok(())
    }

    #[test]
    fn should_format_multiple_usages_with_html() {
        let note = model::Note {
            usages: vec!["oneline usage".to_owned(), "multiline\nusage".to_owned()],
            ..some_note!()
        };
        let record = StringRecord::from(&note);
        let col = record.into_iter().find(|el| el.contains("usage"));
        assert_eq!(
            col,
            Some(
                "<p>
oneline usage
</p>
<p>
multiline
usage
</p>"
            )
        );
    }
}
