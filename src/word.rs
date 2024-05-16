#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Word {
    pub base: Vec<String>,
    pub desc: String, // between [] in json, dislpay to give context
}

impl Word {
    pub fn new(one: impl Into<String>) -> self::Word {
        Word {
            base: vec![one.into()],
            ..Default::default()
        }
    }
    pub fn new_list(list: impl Into<Vec<String>>) -> self::Word {
        Word {
            base: list.into(),
            ..Default::default()
        }
    }
}

impl Into<Word> for &str {
    fn into(self) -> Word {
        Word {
            base: vec![self.into()],
            ..Default::default()
        }
    }
}
impl Into<Word> for String {
    fn into(self) -> Word {
        Word {
            base: vec![self.into()],
            ..Default::default()
        }
    }
}
impl Into<Word> for Vec<&str> {
    fn into(self) -> Word {
        Word {
            base: self.iter().map(|word| String::from(*word)).collect(),
            ..Default::default()
        }
    }
}
impl Into<Word> for Vec<String> {
    fn into(self) -> Word {
        Word {
            base: self,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum GramClass {
    Adverb,
    Noun,
    Verb,
    #[default]
    Other,
}

impl Into<GramClass> for &str {
    fn into(self) -> GramClass {
        match self {
            "Adverb" | "adverb" | "Adv" | "adv" | "Adverbe" | "adverbe" => GramClass::Adverb,
            "Noun" | "noun" | "Nom" | "nom" | "n" => GramClass::Noun,
            "Verb" | "verb" | "Verbe" | "verbe" | "v" => GramClass::Verb,
            _ => GramClass::Other,
        }
    }
}
impl Into<GramClass> for &String {
    fn into(self) -> GramClass {
        self.as_str().into()
    }
}
