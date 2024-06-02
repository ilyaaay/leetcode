fn main() {
    score_of_string("hello".to_string());
}

fn score_of_string(s: String) -> i32 {
    let mut value = 0;

    let char_vec = s.chars().collect::<Vec<char>>();

    for i in 0..s.len() - 1 {
        value += i32::abs(
            char_vec[i].to_ascii_lowercase() as i32 - char_vec[i + 1].to_ascii_lowercase() as i32,
        );
    }

    value
}

#[cfg(test)]
mod test {
    use crate::score_of_string;

    #[test]
    fn test1() {
        let test = "hello".to_string();
        let actual = 13;

        assert_eq!(score_of_string(test), actual);
    }

    #[test]
    fn test2() {
        let test = "zaz".to_string();
        let actual = 50;

        assert_eq!(score_of_string(test), actual);
    }
}
