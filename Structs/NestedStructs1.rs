use std::ops::Add;

struct Address {
    city : String,
    zip: u32,
}

struct Compnay {
    name: String,
    address: Address,
}

fn main() {
    let c = Compnay{
        name: String::from("Rust co"),
        address: Address {
            city: String::from ("Dhaka"),
            zip:1200,
        },
    };


println!("{} is in {}" , c.name , c.address.city);
}
