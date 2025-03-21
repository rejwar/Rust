fn main() {
    let mut x = String::from("Mutable");
    modify(&mut x);
    println!("{}", x);
}

fn modify<'a>(input: &'a mut String) {
    input.push_str(" Updated");
}
