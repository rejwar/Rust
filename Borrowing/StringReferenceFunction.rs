fn PrintString(s :&String) {
    println!("The string is : {}", s);
}

fn main() {
    let MyString:String = String::from("Rust");
    let StringReference: &String = &MyString;

    PrintString(StringReference);
    println!("My string is still : {}", MyString);
}
