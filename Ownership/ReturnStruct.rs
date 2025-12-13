// FileName: ReturnStruct.rs
struct User {
    name: String,
}
fn new_user() -> User {
    User {
        name: String::from("Me"),
    }
}
fn main() {
    let u = new_user();
}
