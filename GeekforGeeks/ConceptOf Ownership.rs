fn helper() -> Box<i32> {
    let three = Box::new(3);
    three
}

fn main() {
    let my_three = helper();
    println!("The value is: {}", my_three);
}

