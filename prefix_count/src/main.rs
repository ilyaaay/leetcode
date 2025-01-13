fn main() {}

fn _prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.into_iter().filter(|x| x.starts_with(&pref)).count() as i32
}

#[test]
fn test() {
    let input = Vec::from([
        "pay".into(),
        "attention".into(),
        "practice".into(),
        "attend".into(),
    ]);
    assert_eq!(_prefix_count(input, "at".into()), 2);
}
