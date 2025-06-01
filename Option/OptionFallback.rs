fn add_five(x: i32) -> Option<i32> {
    Some(x + 5)
}

fn multiply_by_two(x: i32) -> Option<i32> {
    Some(x * 2)
}

fn main() {
    let Result = Some(10)
        .and_then(add_five)
        .and_then(multiply_by_two);

    println!("Final Value: {:?}", Result);
}
