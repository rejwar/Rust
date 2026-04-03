fn main() {
    let mut val = 5;

    loop {
        println!(" {}", val);

        val -= 1;

        if val == 0 {
            break;
        }
    }
}
