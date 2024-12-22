fn main() {}

fn valid_palindrome(s: String) -> bool {
    s.chars()
        .filter(|x| matches!(x, 'A'..='Z' | 'a'..='z' ))
        .filter_map(|x| match x.to_string().parse::<i32>() {
            Ok(_) => None,
            Err(_) => Some(x),
        })
        .map(|x| x.to_ascii_lowercase())
        .rev()
        .collect::<String>()
        .eq(&s
            .chars()
            .filter(|x| matches!(x, 'A'..='Z' | 'a'..='z' ))
            .filter_map(|x| match x.to_string().parse::<i32>() {
                Ok(_) => None,
                Err(_) => Some(x),
            })
            .map(|x| x.to_ascii_lowercase())
            .collect::<String>())
}

#[test]
fn test() {
    assert!(valid_palindrome("A man, a plan, a canal: Panama".into()));
    assert!(!valid_palindrome("0P".into()))
}
