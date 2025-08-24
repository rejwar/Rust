#[derive(Debug)]

struct Ticket {
    title: String,
    status: String,
}

fn main() {
    let t = Ticket {
        title: "Rust".to_string(),
        status: "Open".to_string(),
    };
    println!("{:?}", t);
}