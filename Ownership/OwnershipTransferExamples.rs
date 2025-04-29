fn give_ownership() -> String {
    let some_string = String::from("Yours");
    some_string  // 
}

fn take_and_give_back(a_string: String) -> String {
    println!("'{}' taken", a_string);
    a_string  //
}

fn main() {
    let s1 = give_ownership();
    println!("s1 is {}", s1);

    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);
    println!("s3 is {}", s3);
}
