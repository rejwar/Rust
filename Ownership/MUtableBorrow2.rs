fn main() {
    let mut s = String::from("Hello");
    add_exclamation(&mut s);
    println!("{}", s)
}

fn add_exclamation(s: &mut String) {
    s.push_str("!!!!");
}
