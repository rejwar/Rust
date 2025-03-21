fn main() {
    let value: Option<i32> = Some(5);
    if value.is_some() {
        println!("The value exists!");
    }
}
