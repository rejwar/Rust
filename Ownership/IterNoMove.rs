fn main() {
    let v = vec![String::from("A"), String::from("B")];

    for item in v.iter() {
        println!("Looking at {}", item)
    }

    println!("THe nuvalue is {:?}", v);
}
