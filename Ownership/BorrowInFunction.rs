fn main()  {
    let mut Data:String = String::from("Fn Borrow");
    ShowImmutable(&Data);
    ShowMutable(& mut Data);
}

fn ShowImmutable(Input: &String) {
    println!("Immutable Fn: {}", Input);
}

fn ShowMutable(Input: &mut String) {
    Input.push_str("Changed");
    println!("Mutable fn : {}", Input);
}