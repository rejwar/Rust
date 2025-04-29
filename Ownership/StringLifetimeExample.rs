fn Longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn LifetimeExample() {
    let s1 = String::from("long");
    let s2 = String::from("short");

    let result = Longest(&s1, &s2);
    println!("Longest: {}", result);
}
