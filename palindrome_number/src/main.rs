fn main() {}

fn is_palindrome(x: i32) -> bool {
    let mut reversed = 0;
    let mut value = x;

    while value > 0 {
        reversed = (reversed << 3) + (reversed << 1) + (value % 10);
        value /= 10;
    }

    x == reversed
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(121));
}

#[test]
#[should_panic]
fn panic_test_is_palindtome() {
    assert!(is_palindrome(120))
}
