// FileName: NoCopyStruct.rs
struct User {
    name: String,
} // String is not Copy
fn main() {
    let u1 = User {
        name: String::from("A"),
    };
    let u2 = u1; // Move
                 // println!("{}", u1.name); // Error
}
