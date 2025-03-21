struct Address {
    city: String,
    country: String,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        address: Address {
            city: String::from("Paris"),
            country: String::from("France"),
        },
    };
    println!("Name: {}, City: {}, Country: {}", person.name, person.address.city, person.address.country);
}
