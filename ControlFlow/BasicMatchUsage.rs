fn BasicMatchExample(x:i32) {
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _=> println!("Something else"),
    }
}

fn main() {
    BasicMatchExample(2);
    BasicMatchExample(5);
}
