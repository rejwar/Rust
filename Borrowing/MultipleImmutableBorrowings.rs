fn main() {
    let Data: String = String::from("Multiple Immutable Access");
    let RefOne: &String = &Data;
    let RefTwo: &String = &Data;
    println!("RefOne: {}", RefOne);
    println!("RefTwo: {}", RefTwo);
}
