struct Account {
    username: String,
    active: bool,
}

fn main() {
    let acc = Account {
        username: String::from("Rifat"),
        active: true,
    };
    let name = acc.username;

    println!("Is active {}", acc.active);

    // println!(" {}", acc);
}
