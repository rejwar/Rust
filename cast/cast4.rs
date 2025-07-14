#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn main() {
    let num = Number::from(30);
    println!("Number: {:?}", num);

    let another_num: Number = 5.into(); // let another_num: Number = Number::from(5);
    println!("Another number: {:?}", another_num);
}