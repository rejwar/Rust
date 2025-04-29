fn main() {
    let mut Numbers: Vec<i32> = vec![10, 20, 30];

    for Number in &mut Numbers {
        *Number += 5;
    }

    println!("Updated Numbers: {:?}", Numbers);
}
