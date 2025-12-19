
struct User {
    name: String,
    age: u32,
    active: bool,
}


let u1 = User {
    name: String::from("Hello"),
    age: 23,
    active: true,
};

println!("{}", u1.name);
