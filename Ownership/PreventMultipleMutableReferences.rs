fn main() {
    let mut Data: String = String::from("Rust");
    let RefOne: &mut String = &mut Data;
    // let RefTwo: &mut String = &mut Data; // ❌ Error! Multiple mutable borrows not allowed

    println!("{}", RefOne);
}
