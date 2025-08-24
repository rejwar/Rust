#[derive(Debug , PartialEq)]
struct Ticket {
    Title: String,
    Status: String,
}

fn main() {
    let T1:Ticket = Ticket {Title: "Rust".to_string(), Status: "Open" .to_string()};
    let T2:Ticket = Ticket { Title: "Rust".to_string(), Status: "Open".to_string() };


    if T1 == T2 {
        println!("Same Ticket");
    } else {
        println!("Different Ticket");
    }
}