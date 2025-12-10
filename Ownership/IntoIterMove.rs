fn main() {
    let v = vec![String::from("A"), String::from("b")];

    for item in v.into_iter() {
        println!("Consumed {}", item);
    }
}
