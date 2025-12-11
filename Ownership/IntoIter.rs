fn main() {
    let v = vec![String::from("A"), String::from("B")];

    for item in v.into_iter() {
        println!("Owned item {}", item);
    }
}
