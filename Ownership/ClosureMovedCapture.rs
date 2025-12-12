fn main() {
    let s = String::from("captured");

    let c = move || {
        println!(" Owned by clsoure {}", s);
    };
    c();
}
