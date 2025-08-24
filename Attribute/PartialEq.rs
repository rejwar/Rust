#[derive(Debug , PartialEq)]

struct Ticket {
    title: String,
    status: String,
}

fn main() {
    let t1 = Ticket {title: "Rust".to_string(), status: "Open".to_string()};
    let t2: Ticket = Ticket { title: "Rust".to_string(), status: "Open".to_string() };

    if t1 == t2  {
        println!("Same ticket ");
    }
}