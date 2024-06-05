use std::{collections::HashMap, iter::Iterator};

fn main() {
    let words = vec!["aab".to_string(), "bccd".to_string(), "bxxy".to_string()];

    find_common_char(words);
}

fn find_common_char(words: Vec<String>) -> Vec<char> {
    let mut common_char_vec = Vec::new();

    let mut first_char_map = HashMap::new();
    let mut second_char_map = HashMap::new();

    for i in words[1].chars() {
        first_char_map.insert(i, i);
    }

    for i in words[2].chars() {
        second_char_map.insert(i, i);
    }

    for i in words[0].chars() {
        if first_char_map.get(&i).is_some() && second_char_map.get(&i).is_some() {
            common_char_vec.push(i);
        }
    }

    // let words_iterator = words[1]
    //     .chars()
    //     .zip(words[2].chars())
    //     .collect::<Vec<(char, char)>>();

    // for (x, y) in words_iterator {
    //     if char_map.get(&x).is_some() {
    //         if char_map.get(&y).is_some() {
    //             common_char_vec.push(*char_map.get(&y).unwrap());
    //         }
    //     }
    // }

    common_char_vec
}

#[cfg(test)]
mod test {
    use crate::find_common_char;

    #[test]
    fn first_test() {
        let test = vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string(),
        ];
        let actual = vec!['e', 'l', 'l'];

        assert_eq!(find_common_char(test), actual);
    }

    #[test]
    fn second_test() {
        let test = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        let actual = vec!['c', 'o'];

        assert_eq!(find_common_char(test), actual);
    }
}
