use std::collections::HashSet;

fn main() {}

fn _find_the_difference(s: String, t: String) -> char {
    *t.chars()
        .collect::<HashSet<char>>()
        .difference(&s.chars().collect::<HashSet<char>>())
        .last()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(_find_the_difference("abcd".into(), "abcde".into()), 'e');
    assert_eq!(_find_the_difference("".into(), "y".into()), 'y');
    assert_eq!(_find_the_difference("a".into(), "aa".into()), 'y');
}
