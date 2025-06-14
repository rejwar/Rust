fn main() {
    let mut name = String::from("Rust");

    let r1 = &name;
    let r2 = &name;
    // let r3 = &mut name; // ❌ Error: cannot borrow as mutable while immutable refs exist

    println!("{}, {}", r1, r2);
}
