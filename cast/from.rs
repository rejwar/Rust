#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(value: i32) -> Self {
        MyNumber { value: num }
    }
}

fn main() {
    let number = MyNumber::from(50);
    println!("Fron {:?}", number);
}