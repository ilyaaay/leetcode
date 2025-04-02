fn main() {}

fn merge_alternately(word1: String, word2: String) -> String {
    let mut s = String::new();

    let mut left = 0;
    let mut right = 0;

    let first = word1.chars().collect::<Vec<char>>();
    let second = word2.chars().collect::<Vec<char>>();

    while left == right {
        s.push(first[left]);
        s.push(second[right]);

        left += 1;
        right += 1;

        if left < right {
            break;
        }

        if left > right {
            break;
        }
    }

    s
}

#[test]
fn test() {
    let (word1, word2) = (String::from("abc"), String::from("123"));
    assert_eq!(merge_alternately(word1, word2), String::from("a1b2c3"));

    let (word1, word2) = (String::from("abcd"), String::from("45"));
}
