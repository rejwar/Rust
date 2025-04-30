fn CreateBox() -> Box<String> {
    Box::new(String::from("Safe Rust"))
}

fn main() {
    let BoxedData: Box<String> = CreateBox();
    let Reference: &String = &BoxedData;

    println!("{}", Reference);
}
