fn main() {
    let v = vec![String::from("A")];

    for item in v.into_iter() {
        println!("{}", item);
    }
}
