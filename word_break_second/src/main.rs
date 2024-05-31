fn main() {
    let result = word_break(
        "iloverustt".to_string(),
        vec!["i".to_string(), "love".to_string(), "rust".to_string()],
    );

    println!("{:?}", result);
}

fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut word_vec = Vec::new();

    for (i, str) in word_dict.into_iter().enumerate() {
        if s.get(i..).is_some() {
            word_vec.push(str);
        }
    }

    word_vec
}

#[cfg(test)]
mod test {
    use crate::word_break;

    #[test]
    fn test() {
        let test_str = String::from("catsanddog");
        let word_dict = Vec::from([
            "cat".to_string(),
            "cats".to_string(),
            "and".to_string(),
            "sand".to_string(),
            "dog".to_string(),
        ]);
        let actual = Vec::from(["cats and dog".to_string(), "cat sand dog".to_string()]);

        assert_eq!(word_break(test_str, word_dict), actual);
    }
}
