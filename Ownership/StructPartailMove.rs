struct Data {
    a: String,
    b: String,
}

fn main() {
    let d = Data {
        a: String::from("A"),
        b: String::from("B"),
    };

    let a_part = d.a;

    println!("b is still here {}", d.b);
}
