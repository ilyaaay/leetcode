// impl in-place algorithm with O(1) for reverse string

fn main() {}

fn _reverse_string(s: &mut Vec<char>) {
    let mut length = s.len() - 1;
    let mut i = 0;

    while length > i {
        let el = s[i];
        s[i] = s[length];
        s[length] = el;
        i += 1;
        length -= 1;
    }
}

fn _reverse_string_with_swap(s: &mut Vec<char>) {
    let mut length = s.len() - 1;
    let mut i = 0;

    while length > i {
        s.swap(i, length);
        i += 1;
        length -= 1;
    }
}

fn _fast_reverse(s: &mut Vec<char>) {
    s.reverse()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_reverse_string() {
        let mut test = vec!['h', 'e', 'l', 'l', 'o'];
        let actual = vec!['o', 'l', 'l', 'e', 'h'];

        _reverse_string(&mut test);

        assert_eq!(test, actual);
    }

    #[test]
    fn test_reverse_string_with_swap() {
        let mut test = vec!['h', 'e', 'l', 'l', 'o'];
        let actual = vec!['o', 'l', 'l', 'e', 'h'];

        _reverse_string_with_swap(&mut test);

        assert_eq!(test, actual);
    }

    #[test]
    fn test_fast_reverse() {
        let mut test = vec!['h', 'e', 'l', 'l', 'o'];
        let actual = vec!['o', 'l', 'l', 'e', 'h'];

        _fast_reverse(&mut test);

        assert_eq!(test, actual);
    }
}
