fn main() {
    for i in 1..6 {
        if 1 % 2 == 0 {
            continue;
        }

        println!(" {} ", i);
    }
}
