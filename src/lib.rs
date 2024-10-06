use json::JsonValue;

pub mod word;
pub use word::*;
pub mod english;
pub mod french;
pub mod german;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Entry(pub Word, pub Word, pub GramClass);

impl Entry {
    pub fn get(&self, element: usize) -> String {
        let word = match element {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        let len = word.base.len();
        let mut string = String::new();
        if len >= 2 {
            for i in &word.base[..(&len - 2)] {
                string += format!("{} / ", i).as_str();
            }
        }
        string += &word.base[&len - 1].as_str();
        if ! word.desc.is_empty() {
            string += format!(" [{}]", &word.desc).as_str();
        }
        string
    }
    pub fn correct(&self, answer: &String, element: usize, lang: &Lang) -> f32 {
        let word = match element {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        match *lang {
            Lang::Other if word.base.contains(answer) => 1.,
            Lang::English => english::correct(word, answer, &self.2),
            Lang::French => french::correct(word, answer, &self.2),
            Lang::German => german::correct(word, answer, &self.2),
            _ => 0.,
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
            "English" | "english" | "en" | "en_US" | "en_GB" => Self::English,
            "German" | "Deutsch" | "german" | "deutsch" | "de" | "de_DE" => Self::German,
            "French" | "Français" | "french" | "français" | "Francais" | "francais" | "fr"
            | "fr_FR" => Self::French,
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

pub fn parse(raw: &String) -> Result<([Lang; 2], Vec<Entry>), GramErr> {
    match json::parse(raw.as_str()) {
        Ok(data) if data["lang"].len() == 2 && data["list"].is_array() => {
            let lang1: Lang = data["lang"][0].as_str().unwrap_or("").into();
            let lang2: Lang = data["lang"][1].as_str().unwrap_or("").into();

            let mut list = Vec::new();
            if let JsonValue::Array(unparsed_list) = &data["list"] {
                for unparsed_entry in unparsed_list {
                    match parse_entry(unparsed_entry) {
                        Ok(entry) => list.push(entry),
                        Err(_) => return Err(GramErr::LangErr),
                    }
                }
            }
            Ok(([lang1, lang2], list))
        }
        Err(_) => Err(GramErr::Unknown),
        _ => Err(GramErr::LangErr),
    }
}

fn parse_entry(raw: &JsonValue) -> Result<Entry, GramErr> {
    let mut entry = Entry::default();
    match parse_word(&raw[0]) {
        Ok(word) => entry.0 = word,
        Err(_) => return Err(GramErr::JsonErr),
    }
    match parse_word(&raw[1]) {
        Ok(word) => entry.1 = word,
        Err(_) => return Err(GramErr::JsonErr),
    }
    match &raw[2] {
        JsonValue::Null => entry.2 = GramClass::default(),
        JsonValue::String(gram_class) => entry.2 = gram_class.into(),
        JsonValue::Short(gram_class) => entry.2 = gram_class.as_str().into(),
        _ => return Err(GramErr::JsonErr),
    }
    Ok(entry)
}

fn parse_word(raw: &JsonValue) -> Result<Word, GramErr> {
    match &raw {
        JsonValue::String(word) => Ok(word.into()),
        JsonValue::Short(word) => Ok(word.into()),
        JsonValue::Array(unparsed_words) => {
            let mut words = Vec::new();
            for unparsed_word in unparsed_words {
                match unparsed_word {
                    JsonValue::String(word) => words.push(word.as_str()),
                    JsonValue::Short(word) => words.push(word.as_str()),
                    _ => return Err(GramErr::JsonErr),
                    // _ => words.push(""),
                }
            }
            let words: Vec<String> = words.iter().map(|word| (*word).to_string()).collect();
            Ok(Word::new_list(words))
        }
        _ => return Err(GramErr::JsonErr),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GramErr {
    JsonErr,
    LangErr,
    Unknown,
}
