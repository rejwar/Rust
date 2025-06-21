

#[derive(Debug)]
struct Address {
    city: String,
    zip: u32,
}

#[derive(Debug)]
struct Person {
    name: String,
    address: Address,
}

fn main() {
    let p = Person {
        name: "Md".into(),
        address: Address {
            city: "Dhaka".into(),
            zip: 1200,
        },
    };

    dbg!(&p);
}
