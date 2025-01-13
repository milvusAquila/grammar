use grammar::*;
#[test]
fn german_noun() {
    let noun = Entry("die Kraft".into(), "la force".into(), GramClass::Noun);
    assert_eq!(noun.correct(&"die Kraft".into(), 0, &Lang::German), 1.);
    assert_eq!(noun.correct(&"das Kraft".into(), 0, &Lang::German), 0.5);
    assert_eq!(noun.correct(&"Kraft".into(), 0, &Lang::German), 0.5);
}
