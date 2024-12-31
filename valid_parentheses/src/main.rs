fn main() {
    let s = "()(){}".into();
    let result = is_valid(s);

    println!("{}", result);
}

fn is_valid(s: String) -> bool {
    let mut vec = Vec::new();

    for i in s.chars() {
        if i == '(' || i == '{' || i == '[' {
            vec.push(i);
            vec.rotate_right(1);
        }

        if i == ')' {
            if let Some(&x) = vec.first() {
                if x != '(' {
                    return false;
                }
            }
        }

        if i == '}' {
            if let Some(&x) = vec.first() {
                if x != '{' {
                    return false;
                }
            }
        }

        if i == ']' {
            if let Some(&x) = vec.first() {
                if x != '[' {
                    return false;
                }
            }
        }
    }

    println!("{vec:#?}");

    true
}

#[test]
fn test() {
    // assert!(!is_valid("(".into()));
    assert!(is_valid("()".into()));
    assert!(is_valid("{}".into()));
    assert!(is_valid("[]".into()));
    assert!(is_valid("(){}".into()));
    assert!(is_valid("()[]".into()));
    assert!(is_valid("(){}[]".into()));
    // assert!(is_valid("([])".into()));
    // assert!(!is_valid("((".into()));
}
