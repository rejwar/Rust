fn main() {
    let mut Data: String = String::from("Mutable Test");
    let Ref: &mut String = &mut Data;

    Ref.push_str("Updated");
    println!("After Change {}" , Ref);
}