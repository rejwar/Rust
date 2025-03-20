fn main() {
    let mut counter = 0;
    while counter < 10 {
        if counter == 5 {
            break;
        }
        counter += 1;
    }
    println!("Counter: {}", counter);
}
