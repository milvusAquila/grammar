use grammar::*;
use std::fs;

#[test]
fn entry_test() {
    let entry = Entry(
        Word::new("the solution"),
        Word::new("la solution"),
        GramClass::Noun,
    );
    assert_eq!(
        entry.correct(&String::from("the solution"), 0, &Lang::English),
        1.
    );
}
#[test]
fn parse_test() {
    let raw = String::from(
        r#"{
    "lang": ["english", "french"],
    "list": [
            ["yes", "oui", "adv"],
            ["no", "non", "adverb"],
            ["the work", "le travail", "noun"],
            ["the rust", "la rouille", "noun"],
            ["the solution", "la solution", "noun"],
            ["to rise", ["s'élever", "monter"], "verb"]
    ]
} "#,
    );
    println!("{}", raw);
    let parsed = parse(&raw).unwrap();
    let truth = (
        [Lang::English, Lang::French],
        vec![
            Entry("yes".into(), "oui".into(), GramClass::Adverb),
            Entry("no".into(), "non".into(), GramClass::Adverb),
            Entry("the work".into(), "le travail".into(), GramClass::Noun),
            Entry("the rust".into(), "la rouille".into(), GramClass::Noun),
            Entry("the solution".into(), "la solution".into(), GramClass::Noun),
            Entry(
                "to rise".into(),
                Word::new_list(vec!["s'élever".into(), "monter".into()]),
                GramClass::Verb,
            ),
        ],
    );
    assert_eq!(parsed, truth);
}
#[test]
fn read_file_test() {
    for i in fs::read_dir("assets").expect("Failed to open assets files (should be in /assets/*)") {
        let contents = fs::read_to_string(i.unwrap().path()).unwrap();
        let (langs, database) = parse(&contents).unwrap();
        for i in database {
            println!("{:?}", i);
            for j in &i.0.base {
                assert_eq!(i.correct(&j, 0, &langs[1]), 1.0);
            }
            for j in &i.1.base {
                assert_eq!(i.correct(&j, 1, &langs[0]), 1.0);
            }
        }
    }
}
