fn main() {
    let mut count = 0;
    loop {
        if count == 5 {
            break;
        }
        count += 1;
    }
    println!("Count: {}", count);
}
