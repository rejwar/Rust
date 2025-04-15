fn Factorail (n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n* Factorail(n-1)
    }
}

fn main() {
    println!("Factorail: {}", Factorail(5));
}
