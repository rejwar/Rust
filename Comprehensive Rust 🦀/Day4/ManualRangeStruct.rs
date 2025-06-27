fn main() {
    let r = std::ops::Range { start: 3, end: 7};

    for i in r {
        println!(" {}", i);
    }
}
