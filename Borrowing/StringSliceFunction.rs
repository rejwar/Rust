fn PrintSlice(s: &str) {
    println!("The slice is : {}", s);
}

fn main() {
    let MyString: String = String::from("Hello Rust");
    let StringSlice: &str = &MyString[0..5];
    PrintSlice(StringSlice);

    let StringLiteral: &str = "World";
    PrintSlice(StringLiteral);
}
