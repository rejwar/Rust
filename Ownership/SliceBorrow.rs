fn main() {
    let s = String::from("Hello");

    let hello_hand = &s[0..5];

    let world = &s[6..11];

    println!("First word {}", hello_hand);
    println!("Second word P{}", world);

    println!("Full String {}", s);
}
