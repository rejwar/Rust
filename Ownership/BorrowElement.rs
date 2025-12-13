fn main() {
    let v = vec![String::from("One"), String::from("Two")];

    if let Some(item) = v.get(0) {
        println!("Borrow form vec : {}", item);
    }
}
