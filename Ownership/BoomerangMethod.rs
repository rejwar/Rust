fn main() {
    let s2 = String::from("Hello");
    let s3 = TakesAndBack(s2);

    println!(" s3 has data :{}", s3);
}

fn TakesAndBack(a_string: String) -> String {
    println!("{}", a_string);

    a_string
}
