fn parse_user(input : Option<&str>) {
    let Some(name) = input else {
        println!("No name provided");
        return;
    };
    println!("Hello {}", name);
}

fn main() {
    parse_user(Some("Md"));
    parse_user(None);
}
