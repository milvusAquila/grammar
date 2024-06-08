use crate::{GramClass, Word};
// use regex::Regex;

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    word.base
        .iter()
        .map(|i| match gram_class {
            GramClass::Noun => {
                let mut score = 0.;
                if ["der ", "die ", "das "].contains(&&i[..4]) {
                    if answer == i {
                        // right answer
                        score = 1.;
                    } else if answer == &i[4..] {
                        // right answer without genre
                        score = 0.5
                    }
                } else if answer == i {
                    score = 1.;
                }
                score
            }
            _ if i.eq_ignore_ascii_case(answer) => 1.,
            _ => 0.,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}
