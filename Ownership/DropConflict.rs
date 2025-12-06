fn main() {
    let s = String::from("hello");
    let r = &mut s.clone();

    drop(s);

    r.push_str("World");
}
