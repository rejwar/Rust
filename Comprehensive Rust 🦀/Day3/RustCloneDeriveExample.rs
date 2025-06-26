#[derive(Clone, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let u1 = User{ name: "Md".into(), age: 30};
    let u2 = u1.clone();

    println!("{:?} {:?}", u1, u2);
}
