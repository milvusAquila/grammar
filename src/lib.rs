use json::{self, JsonValue};

pub mod word;
pub use word::*;
pub mod english;
pub mod french;
pub mod german;
// pub use english;
// pub use french;
// pub use german;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Entry(pub Word, pub Word, pub GramClass);

impl Entry {
    pub fn get(&self, lang: usize) -> String {
        let word = match lang {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        match word {
            Word::One(content) => content.to_string(),
            Word::List(content) => {
                let mut formatted = String::new();
                for i in &content[..content.len() - 1] {
                    formatted += format!("{} / ", i).as_str();
                }
                formatted += content[content.len()].as_str();
                formatted
            }
        }
    }
    pub fn correct(&self, answer: &String) -> f32 {
        // TODO: add some grammar tolerences (`to` or not before verb)
        match &self.0 {
            Word::One(word) => {
                if word == answer {
                    1.
                } else {
                    0.
                }
            }
            Word::List(words) => {
                if words.contains(answer) {
                    1.
                } else {
                    0.
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Lang {
    English,
    French,
    German,
    Other,
}

impl<'a> Into<&'a str> for Lang {
    fn into(self) -> &'a str {
        match self {
            Self::English => "English",
            Self::German => "Deutsch",
            Self::French => "Français",
            Self::Other => "Other",
        }
    }
}

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        match value {
            "English" | "english" => Self::English,
            "German" | "Deutsch" | "german" | "deutsch" => Self::German,
            "French" | "Français" | "french" | "français" | "Francais" | "francais" => {
                Self::French
            }
            _ => Self::Other,
        }
    }
}
impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Lang::English => "English",
            Lang::German => "Deutsch",
            Lang::French => "Français",
            Lang::Other => "Other",
        };
        write!(f, "{}", string)
    }
}
/* impl From<Lang> for String {
    fn from(value: Lang) -> Self {
        value.into()
    }
} */

pub fn parse(raw: &String) -> Result<([Lang; 2], Vec<Entry>), json::Error> {
    match json::parse(raw.as_str()) {
        Ok(data) if data["lang"].len() == 2 && data["list"].is_array() => {
            let lang1: Lang = data["lang"][0].as_str().unwrap_or("").into();
            let lang2: Lang = data["lang"][1].as_str().unwrap_or("").into();

            let mut list = Vec::new();
            if let JsonValue::Array(unparsed_list) = &data["list"] {
                for unparsed_entry in unparsed_list {
                    if !unparsed_entry[2].is_string() {
                        return Err(json::Error::UnexpectedEndOfJson);
                    }
                    let mut entry = Entry::default();
                    match &unparsed_entry[0] {
                        JsonValue::String(word) => entry.0 = Word::new(word),
                        JsonValue::Short(word) => entry.0 = Word::new(word.as_str()),
                        JsonValue::Array(unparsed_words) => {
                            let mut words = Vec::new();
                            for unparsed_work in unparsed_words {
                                match unparsed_work {
                                    JsonValue::String(word) => words.push(word.as_str()),
                                    JsonValue::Short(word) => words.push(word.as_str()),
                                    _ => return Err(json::Error::UnexpectedEndOfJson),
                                }
                            }
                            let words: Vec<String> =
                                words.iter().map(|word| (*word).to_string()).collect();
                            entry.0 = Word::new_list(words);
                        }
                        _ => return Err(json::Error::UnexpectedEndOfJson),
                    }
                    match &unparsed_entry[1] {
                        JsonValue::String(word) => entry.1 = Word::new(word),
                        JsonValue::Short(word) => entry.1 = Word::new(word.as_str()),
                        JsonValue::Array(unparsed_words) => {
                            let mut words = Vec::new();
                            for unparsed_work in unparsed_words {
                                match unparsed_work {
                                    JsonValue::String(word) => words.push(word.as_str()),
                                    JsonValue::Short(word) => words.push(word.as_str()),
                                    _ => return Err(json::Error::UnexpectedEndOfJson),
                                }
                            }
                            let words: Vec<String> =
                                words.iter().map(|word| (*word).to_string()).collect();
                            entry.1 = Word::new_list(words);
                        }
                        _ => return Err(json::Error::UnexpectedEndOfJson),
                    }
                    match &unparsed_entry[2] {
                        JsonValue::String(gram_class) => entry.2 = gram_class.into(),
                        JsonValue::Short(gram_class) => entry.2 = gram_class.as_str().into(),
                        _ => return Err(json::Error::UnexpectedEndOfJson),
                    }
                    list.push(entry);
                }
            }
            Ok(([lang1, lang2], list))
        }
        Err(err) => Err(err),
        _ => Err(json::Error::UnexpectedEndOfJson),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_test() {
        let entry = Entry(
            Word::new("the solution"),
            Word::new("la solution"),
            GramClass::Noun,
        );
        assert_eq!(entry.correct(&String::from("the solution")), 1.0);
    }
    #[test]
    fn parse_test() {
        let raw = String::from(
            "{
    \"lang\": [\"english\", \"french\"],
    \"list\": [
            [\"yes\", \"oui\", \"adv\"],
            [\"no\", \"non\", \"adverb\"],
            [\"the work\", \"le travail\", \"noun\"],
            [\"the rust\", \"la rouille\", \"noun\"],
            [\"the solution\", \"la solution\", \"noun\"],
            [\"to rise\", [\"s'élever\", \"monter\"], \"verb\"]
    ]
} ",
        );
        // println!("{}", raw);
        let parsed = parse(&raw).unwrap();
        let truth = (
            [Lang::English, Lang::French],
            vec![
                Entry("yes".into(), "oui".into(), GramClass::Adverb),
                Entry("no".into(), "non".into(), GramClass::Adverb),
                Entry("the work".into(), "le travail".into(), GramClass::Noun),
                Entry("the rust".into(), "la rouille".into(), GramClass::Noun),
                Entry("the solution".into(), "la solution".into(), GramClass::Noun),
                Entry("to rise".into(), Word::new_list(vec!["s'élever".into(), "monter".into()]), GramClass::Verb),
            ],
        );
        assert_eq!(parsed, truth);
    }
}
