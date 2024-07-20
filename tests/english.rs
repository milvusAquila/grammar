use grammar::*;
#[test]
fn english_verb() {
    let verb = Entry("to rise".into(), "s'élever".into(), GramClass::Verb);
    assert_eq!(verb.correct(&"to rise".into(), 0, &Lang::English), 1.);
    assert_eq!(verb.correct(&"rise".into(), 0, &Lang::English), 1.);
    assert_eq!(verb.correct(&"rse".into(), 0, &Lang::English), 0.);
}
#[test]
fn english_noun() {
    let noun = Entry("the solution".into(), "la solution".into(), GramClass::Noun);
    assert_eq!(noun.correct(&"the solution".into(), 0, &Lang::English), 1.);
    assert_eq!(noun.correct(&"solution".into(), 0, &Lang::English), 1.);
    assert_eq!(noun.correct(&"solutio".into(), 0, &Lang::English), 0.);
}
