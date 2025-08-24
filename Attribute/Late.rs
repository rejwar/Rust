#[derive(Debug)]
struct Ticket {
    Titile: String,
    Status: String,
}

fn main() {
    let T1 = Ticket {Titile: "Rust".to_string(), Status: "Open". to_string()};
    println!("{:?}", T1);
}