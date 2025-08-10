#[derive(Debug)]
struct Profile {
    name: String,
    email: Option<String>,
}

fn main() {
    let p1 = Profile {name: "Nadia".into(), email :None};
    let p2 = Profile { name: "Irfan".into(),email: Some ("Irafna@gmail.com".into())};
    println!("{:?} , {:?}", p1 ,p2);
}