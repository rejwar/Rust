fn main() {
    let v = vec![String::from("A"), String::from("B")];

    for x in v.iter() {
        println!("Borrowed {}", x);
    }

    println!("Vec safe {:?}", v);
}
