use std::ops::Index;

fn main() {
    sililar_sentence("My name is Ilya".into(), "1".into())
}

fn sililar_sentence(s1: String, s2: String) -> () {
    let mut x = [(s1.len(), s1), (s2.len(), s2)];
    x.sort();

    let s1 = x.index(1);
    let s2 = x.index(0);
}
