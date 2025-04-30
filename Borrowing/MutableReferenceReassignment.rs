fn main() {
    let mut coffee:String = String::from("Mocha");
    let a: &mut String = &mut coffee;
    println!("{a}");
    let b: &mut String = a;
    println!("{}",b);
}
