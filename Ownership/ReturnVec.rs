// FileName: ReturnVec.rs
fn get_list() -> Vec<String> {
    vec![String::from("A")]
}
fn main() {
    let mut list = get_list();
    list.push(String::from("B"));
}
