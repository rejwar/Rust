fn main() {
    let s = String::from("Cross Border ");
    pass_border(s);
}

fn pass_border(val: String) {
    println!("Received {}", val);
}
