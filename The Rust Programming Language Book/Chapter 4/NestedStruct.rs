struct Address {
    city: String,
    zip: u32,
}

struct User {
    name: String,
    address: Address,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        address: Address {
            city: String::from("Dhaka"),
            zip: 1207,
        },
    };

    println!("{} lives in {}, {}", user.name, user.address.city, user.address.zip);
}
