struct Developer {
    name: String,
    lang: String,
}

fn maian() {
    let dev = Developer {
        name: String::from("Rifat"),
        lang: String::from("Rust"),
    };

    let moved_name = dev.name;
    println!(" Lang is valid {}", dev.lang);
}
