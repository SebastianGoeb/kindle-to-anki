mod api {
    use serde::{Deserialize, Deserializer, Serialize};

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Response(Vec<Entry>);

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Entry {
        meta: Meta,
        #[serde(rename = "hwi")]
        headword_information: HeadwordInformation,
        // functional label: https://dictionaryapi.com/products/json#sec-2.fl
        #[serde(rename = "fl")]
        grammatical_function: String,
        // definition section: https://dictionaryapi.com/products/json#sec-2.sense-struct
        #[serde(rename = "def")]
        definitions: Vec<Definition>,
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Meta {
        id: String,
        uuid: String,
        sort: String,
        src: String,
        section: String,
        stems: Vec<String>,
        offensive: bool,
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct HeadwordInformation {
        #[serde(rename = "hw")]
        headword: String,
        #[serde(rename = "prs")]
        pronunciations: Option<Vec<Pronunciation>>,
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Pronunciation {
        #[serde(rename = "mw")]
        merriam_webster: Option<String>,
        #[serde(rename = "l")]
        label_before: Option<String>,
        #[serde(rename = "l2")]
        label_after: Option<String>,
        #[serde(rename = "pun")]
        punctuation: Option<String>,
        sound: Option<Sound>,
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Sound {
        // base filename for audio playback
        audio: String,
        // ref ignored
        // stat ignored
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Definition {
        #[serde(rename = "vd")]
        verb_divider: Option<String>,
        #[serde(rename = "sseq")]
        sense_sequence: SenseSequence,
    }

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct SenseSequence(Vec<Vec<SenseWrapper>>);

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct SenseWrapper(String, Sense);

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct Sense {
        #[serde(rename = "dt")]
        defining_text: Vec<DefiningTextWrapper>,
        #[serde(rename = "sn")]
        sense_number: Option<String>,
    }

    #[derive(Debug, Serialize, PartialEq)]
    enum DefiningTextWrapper {
        DefiningText(String),
        VerbalIllustrations(Vec<String>),
        Unknown,
    }

    impl<'de> Deserialize<'de> for DefiningTextWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if let serde_json::Value::Array(contents) = Deserialize::deserialize(deserializer)? {
                match contents[0].as_str() {
                    Some("text") => {
                        return Ok(DefiningTextWrapper::DefiningText(
                            contents[1].as_str().unwrap().to_owned(),
                        ))
                    }
                    Some("vis") => {
                        return Ok(DefiningTextWrapper::VerbalIllustrations(
                            contents[1]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| {
                                    item.as_object()
                                        .unwrap()
                                        .get("t")
                                        .unwrap()
                                        .as_str()
                                        .unwrap()
                                        .to_owned()
                                })
                                .collect(),
                        ))
                    }
                    _ => Ok(DefiningTextWrapper::Unknown),
                }
            } else {
                Ok(DefiningTextWrapper::Unknown)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use pretty_assertions::assert_eq;
        use std::{error::Error, fs};

        use super::*;

        #[test]
        fn should_parse_response() -> Result<(), Box<dyn Error>> {
            let path = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/test/merriam_webster/collegiate/voluminous_response.json"
            );

            let response = fs::read_to_string(path)?;

            let response: Response = serde_json::from_str(&response)?;

            assert_eq!(
                response,
                Response(vec![Entry {
                    meta: Meta {
                        id: "voluminous".to_owned(),
                        uuid: "0d01b967-971f-4ec5-8fe0-10513d29c39b".to_owned(),
                        sort: "220130400".to_owned(),
                        src: "collegiate".to_owned(),
                        section: "alpha".to_owned(),
                        stems: vec![
                            "voluminous".to_owned(),
                            "voluminously".to_owned(),
                            "voluminousness".to_owned(),
                            "voluminousnesses".to_owned()
                        ],
                        offensive: false
                    },
                    headword_information: HeadwordInformation {
                        headword: "vo*lu*mi*nous".to_owned(),
                        pronunciations: Some(vec![Pronunciation {
                            merriam_webster: Some(
                                "v\u{0259}-\u{02c8}l\u{00fc}-m\u{0259}-n\u{0259}s".to_owned()
                            ),
                            sound: Some(Sound {
                                audio: "volumi02".to_owned()
                            }),
                            ..Pronunciation::default()
                        }])
                    },
                    grammatical_function: "adjective".to_owned(),
                    definitions: vec![Definition {
                        verb_divider: None,
                        sense_sequence: SenseSequence(vec![
                            vec![
                                SenseWrapper(
                                    "sense".to_owned(),
                                    Sense {
                                        defining_text: vec![
                                            DefiningTextWrapper::DefiningText("{bc}having or marked by great {a_link|volume} or bulk {bc}{sx|large||} ".to_owned()),
                                            DefiningTextWrapper::VerbalIllustrations(vec!["long {wi}voluminous{/wi} tresses".to_owned()])
                                        ],
                                        sense_number: Some("1 a".to_owned())
                                    }
                                ),
                                SenseWrapper(
                                    "sense".to_owned(),
                                    Sense {
                                        defining_text: vec![
                                            DefiningTextWrapper::DefiningText("{bc}{sx|numerous||} ".to_owned()),
                                            DefiningTextWrapper::VerbalIllustrations(vec!["trying to keep track of {wi}voluminous{/wi} slips of paper".to_owned()])
                                        ],
                                        sense_number: Some("b".to_owned())
                                    }
                                ),
                            ],
                            vec![
                                SenseWrapper(
                                    "sense".to_owned(),
                                    Sense {
                                        defining_text: vec![
                                            DefiningTextWrapper::DefiningText("{bc}filling or capable of filling a large volume or several {a_link|volumes} ".to_owned()),
                                            DefiningTextWrapper::VerbalIllustrations(vec!["a {wi}voluminous{/wi} literature on the subject".to_owned()])
                                        ],
                                        sense_number: Some("2 a".to_owned())
                                    }
                                ),
                                SenseWrapper(
                                    "sense".to_owned(),
                                    Sense {
                                        defining_text: vec![
                                            DefiningTextWrapper::DefiningText("{bc}writing or speaking much or at great length ".to_owned()),
                                            DefiningTextWrapper::VerbalIllustrations(vec!["a {wi}voluminous{/wi} correspondent".to_owned()])
                                        ],
                                        sense_number: Some("b".to_owned())
                                    }
                                ),
                            ],
                            vec![SenseWrapper(
                                "sense".to_owned(),
                                Sense {
                                    defining_text: vec![
                                        DefiningTextWrapper::DefiningText("{bc}consisting of many folds, coils, or convolutions {bc}{sx|winding|winding:2|}".to_owned()),
                                        
                                    ],
                                    sense_number: Some("3".to_owned())
                                }
                            ),],
                        ])
                    }]
                }])
            );

            Ok(())
        }
    }
}
