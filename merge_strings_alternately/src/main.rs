fn main() {}

fn merge_alternately(word1: String, word2: String) -> String {
    let word = format!("{word1}{word2}");
    let mut left = 0;
    let mut right = word.len() - 1;

    let mut result = String::new();

    while left <= right {}

    result
}

#[test]
fn test() {
    let (word1, word2) = (String::from("abc"), String::from("123"));
    assert_eq!(merge_alternately(word1, word2), String::from("a1b2c3"));

    let (word1, word2) = (String::from("abcd"), String::from("45"));
}
