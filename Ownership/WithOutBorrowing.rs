fn Takes_And_Returs_Ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = Takes_And_Returs_Ownership(s1);

    println!("{}", s2);
}