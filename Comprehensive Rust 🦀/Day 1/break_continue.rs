fn main() {
    for i in 1..=5 {
        if i ==3 {
            continue;
        }
        if i == 5 {
            break;
        }
        println!(" i = {}", i);
    }
}
