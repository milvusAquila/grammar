use crate::{GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    word.base
        .iter()
        .map(|i| match gram_class {
            GramClass::Verb if answer == i || (&i[..3] == "to " && answer == &i[3..]) => 1.,
            GramClass::Noun if answer == i || (&i[..4] == "the " && answer == &i[4..]) => 1.,
            GramClass::Noun if answer == i || (&i[..2] == "a " && answer == &i[2..]) => 1.,
            _ if i.eq_ignore_ascii_case(answer) => 1.,
            _ => 0.,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}
