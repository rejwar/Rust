// FileName: ReturnTuple.rs
fn get_pair() -> (String, i32) {
    (String::from("A"), 1)
}
fn main() {
    let (s, i) = get_pair();
}
