fn main() {
    let mut list = vec![String::from("Top item")];

    let item = list.pop().unwrap();
    println!("Popped {}", item);
}
