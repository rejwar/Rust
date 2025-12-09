fn main() {
    let received = make_string();

    println!(" I own {}", received);
}

fn make_string() -> String {
    let s = String::from("Return me ");
    s
}
