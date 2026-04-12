fn HalvingLoop(mut n: i32) {
    while n > 1 {
        println!(" {}", n);
        n = n / 2;
    }
}

fn main() {
    HalvingLoop(100);
}
