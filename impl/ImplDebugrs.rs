#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {name: "Md" . to_string(), age: 25};
    println!("{:?}", p);
}