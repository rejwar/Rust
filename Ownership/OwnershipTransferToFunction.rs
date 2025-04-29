fn TakeOwnership(SomeStrig:String) {
    println!("{} Taken", SomeStrig);
}

fn main() {
    let s = String::from("Hello");
    TakeOwnership(s);
}
