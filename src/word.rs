#[derive(Debug)]
pub enum Word {
    One(String),
    List(Vec<String>),
}

impl Word {
    pub fn new(one: impl Into<String>) -> self::Word {
        Word::One(one.into())
    }
    pub fn new_list(list: impl Into<Vec<String>>) -> self::Word {
        Word::List(list.into())
    }
}

impl Default for Word {
    fn default() -> Self {
        Word::One(String::default())
    }
}
impl Into<Word> for &str {
    fn into(self) -> Word {
        Word::One(String::from(self))
    }
}
impl Into<Word> for String {
    fn into(self) -> Word {
        Word::One(self)
    }
}
impl Into<Word> for Vec<&str> {
    fn into(self) -> Word {
        Word::List(self.iter().map(|word| String::from(*word)).collect())
    }
}
impl Into<Word> for Vec<String> {
    fn into(self) -> Word {
        Word::List(self)
    }
}

#[derive(Debug, Default)]
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
            "Noun" | "noun" | "Nom" | "nom" => GramClass::Noun,
            "Verb" | "verb" | "Verbe" | "verbe" => GramClass::Verb,
            _ => GramClass::Other,
        }
    }
}
impl Into<GramClass> for &String {
    fn into(self) -> GramClass {
        self.as_str().into()
    }
}
