struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair {
        first: 10,
        second: "Rust",
    };
    println!("First: {}, Second: {}", pair.first, pair.second);
}
