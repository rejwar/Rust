struct Pair<T, U> {
    First: T,
    Second: U,
}

fn main() {
    let Data = Pair::<i32, String> { First: 10, Second: String::from("Rust") };
    println!("Pair: {}, {}", Data.First, Data.Second);
}
