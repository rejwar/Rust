fn take_ownerhip(s: String) {
    println!("Got string {}" , s);
}

fn main() {
    let s1= String::from("Rust");
    take_ownerhip(s1);
}
