fn main() {
    let v = vec![String::from("A"), String::from("B")];

    let item_ref = &v[0];
    println!("Borrowed {}", item_ref);
}
