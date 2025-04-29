fn PrintLength(name:&String) {
    println!("Length : {}", name.len());
}

fn ImmutableBorrow() {
    let name= String::from("Rustacean");
    PrintLength(&name);

    println!("Still valid : {}", name);
}
