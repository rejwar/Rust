fn main() {
    let mut Name = String::from("Rust");

    let Ref1 = &mut Name;
    // let Ref2 = &mut Name; ❌ Error! Multiple mutable borrowing.

    println!("Reference 1: {}", Ref1);
}
