struct Developer {
    name: String,
    lang: String,
}

fn main() {
    let dev = Developer {
        name: String::from("Hello"),
        lang: String::from("Rifat"),
    };

    let my_name = dev.name;
    println!("Name moved {}", my_name);
    println!("Name moved is {}", dev.lang);
}
