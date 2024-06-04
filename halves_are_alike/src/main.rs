fn main() {}

fn halves_are_alike(s: String) -> bool {
    let (x, y) = {
        let (x, y) = s.split_at(s.len() / 2);
        (x.chars(), y.chars())
    };

    let predicate =
        |x: &char| -> bool { ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(x) };

    x.filter(|ch| predicate(ch)).count() == y.filter(|ch| predicate(ch)).count()
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
