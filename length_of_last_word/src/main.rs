fn main() {}

fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .chars()
        .collect::<String>()
        .split_whitespace()
        .last()
        .unwrap()
        .len() as i32
}

#[cfg(test)]
mod test {
    use crate::length_of_last_word;

    #[test]
    fn first_test() {
        let test = "Hello world".to_string();
        assert_eq!(length_of_last_word(test), 5);
    }

    #[test]
    fn second_test() {
        let test = "   fly me   to   the moon  ".to_string();
        assert_eq!(length_of_last_word(test), 4);
    }
}
