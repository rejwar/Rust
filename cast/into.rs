use core::num;

#[derive(Debug)]
struct MyNumber{
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(value: i32) -> Self {
        MyNumber { value: num }
    }
}

fn main() {
    let number: MyNumber = 75.into();
    println!("Into {:?}", number)
}