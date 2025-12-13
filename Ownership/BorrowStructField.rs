struct User {
    name: String,
}

fn main() {
    let u = User {
        name: String::from("Rifat"),
    };
    print_name(&u.name);
}

fn print_name(n: &String) {
    println!("User {}", n);
}
