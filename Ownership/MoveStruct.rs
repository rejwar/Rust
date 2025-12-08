struct Developer {
    username: String,
    skill_level: u32,
}

fn main() {
    let dev_profile = Developer {
        username: String::from("Rustaceans"),
        skill_level: 99,
    };

    promote_developer(dev_profile);
}

fn promote_developer(d: Developer) {
    println!("{}", d.username);
}
