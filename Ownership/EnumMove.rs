// FileName: EnumMove.rs
enum Data {
    Text(String),
}
fn main() {
    let d1 = Data::Text(String::from("Moved"));
    let d2 = d1;
    // println!("{:?}", d1); // Error
}
