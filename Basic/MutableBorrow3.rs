fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!(" {}", s);
}

fn change(str: &mut String) {
    str.push_str("World");
}
