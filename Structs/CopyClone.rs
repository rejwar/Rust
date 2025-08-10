#[derive(Debug , Clone)]

struct Tag {
    name: String,
}

fn main() {
    let t1 = Tag {name: "rust".into()};
    let t2 = t1.clone();
    println!("t1 = {:?}, t2 = {:?}" ,t1 , t2);


}