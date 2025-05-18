fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("After change : {}", s);
}

fn change (some_string: &mut String) {
    some_string.push_str(", world");
}
