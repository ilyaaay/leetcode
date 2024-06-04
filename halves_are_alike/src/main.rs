fn main() {}

fn halves_are_alike(s: String) -> bool {
    let vowels_vec = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let (mut c1, mut c2) = (0, 0);

    let vec = {
        let (x, y) = s.split_at(s.len() / 2);
        x.chars().zip(y.chars()).collect::<Vec<_>>()
    };

    for (x, y) in vec.iter() {
        if vowels_vec.contains(x) {
            c1 += 1;
        }

        if vowels_vec.contains(y) {
            c2 += 1;
        }
    }

    if c1 == c2 {
        return true;
    }

    false
}

#[cfg(test)]
mod test {
    use crate::halves_are_alike;

    #[test]
    fn test1() {
        let test = "book".to_string();
        assert!(halves_are_alike(test));
    }

    #[test]
    fn test2() {
        let test = "textbook".to_string();
        assert_eq!(halves_are_alike(test), false);
    }

    #[test]
    fn test3() {
        let test = "AbCdEfGh".to_string();
        assert_eq!(halves_are_alike(test), true);
    }
}
