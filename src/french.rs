use crate::{GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    if answer.is_empty() {
        return 0.;
    }
    word.base
        .iter()
        .map(|i| match gram_class {
            _ if i.eq_ignore_ascii_case(answer) => 1.,
            GramClass::Noun => {
                let mut score = 0.;
                if ["le ", "la "].contains(&&i[..3]) {
                    if answer.len() >= 4 {
                        if (&i[3..]).eq_ignore_ascii_case(&answer[3..]) {
                            score = 0.5;
                        }
                    }
                    if (&i[3..]).eq_ignore_ascii_case(answer) {
                        score = 0.5;
                    }
                }
                score
            }
            _ => 0.,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}
