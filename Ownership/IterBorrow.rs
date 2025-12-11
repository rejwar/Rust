fn main() {
    let v = vec![String::from("a"), String::from("B")];

    for item in v.iter() {
        println!("Borrowed item {}", item);
    }

    println!("Vector is safe {:?}", v);
}
