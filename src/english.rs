use ordered_float::NotNan;

use crate::{GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    word.base
        .iter()
        .map(|i| {
            println!("'{}' '{}' '{}'", i, &i[3..], answer);
            match gram_class {
            GramClass::Verb if answer == i || (&i[..3] == "to " && answer == &i[3..]) => 1.,
            GramClass::Noun if answer == i || (&i[..4] == "the " && answer == &i[4..]) => 1.,
            GramClass::Noun if &i[..2] == "a " && (answer == i || answer == &i[..2]) => 1.,
            _ => 0.,
        } })
        // .fold(0.0f32, |max, &val| if val > max { val } else { max })
        .map(NotNan::new)
        .flatten()
        .max()
        .map(NotNan::into_inner)
        .unwrap()
}
// pub fn english_verb(word: &Word, answer: &String) -> f32 {
//     match word {
//         Word::One(word) if &word[..3] == "to " && answer == &word[..3] || answer == word => 1.,
//         _ => 0.,
//     }
// }

mod tests {
    // use super::*;
    #[test]
    fn english_verb() {
        use crate::*;
        let verb = Entry("to rise".into(), "s'élever".into(), GramClass::Verb);
        assert_eq!(verb.correct(&"to rise".into(), 0, &Lang::English), 1.);
        assert_eq!(verb.correct(&"rise".into(), 0, &Lang::English), 1.);
        let noun = Entry("the solution".into(), "la solution".into(), GramClass::Noun);
        assert_eq!(noun.correct(&"the solution".into(), 0, &Lang::English), 1.);
        assert_eq!(noun.correct(&"solution".into(), 0, &Lang::English), 1.);
    }
}
