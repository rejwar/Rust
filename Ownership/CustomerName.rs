#[derive(Debug)]

struct Ticket {
    id: u32,
    customer_name: Option<String>,
}

fn main() {
    let mut my_ticket = Ticket {
        id: 505,
        customer_name: Some(String::from("Rifat")),
    };

    let served_customer = my_ticket.customer_name.take();

    match served_customer {
        Some(name) => println!("Now serving {}", name),
        None => println!("No customer found"),
    }

    println!("{:?}", my_ticket);
}
