fn main() {
    let x = "tsur".to_string();
    let ch = 'r';
    println!("{}", reverse_word_prefix(x, ch));
}

fn reverse_word_prefix(word: String, ch: char) -> String {
    match word.split_once(ch) {
        Some((x, y)) => format!("{}{}{}", ch, x.chars().rev().collect::<String>(), y),
        None => word,
    }
}

#[cfg(test)]
mod test {
    use crate::reverse_word_prefix;

    #[test]
    fn first() {
        let word = "abcdefd".to_string();
        let ch = 'd';
        let actual = "dcbaefd".to_string();
        assert_eq!(reverse_word_prefix(word, ch), actual);
    }

    #[test]
    fn second() {
        let word = "xyxzxe".to_string();
        let ch = 'z';
        let actual = "zxyxxe".to_string();
        assert_eq!(reverse_word_prefix(word, ch), actual);
    }

    #[test]
    fn third() {
        let word = "abcd".to_string();
        let ch = 'z';
        let actual = "abcd".to_string();
        assert_eq!(reverse_word_prefix(word, ch), actual);
    }
}
