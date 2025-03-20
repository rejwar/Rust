fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    }
    println!("Counter: {}", counter);
}
