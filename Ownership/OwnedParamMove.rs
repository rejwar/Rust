fn main() {
    let my_string = String::from("Take me");

    take_ownership(my_string);
}

fn take_ownership(s: String) {
    println!(" I own it {}", s);
}
