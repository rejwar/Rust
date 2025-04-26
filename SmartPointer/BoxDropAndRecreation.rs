fn main() {
    let BoxNumber = Box::new(42);

    println!("BoxedNumber : {} ", BoxNumber);

    drop(BoxNumber);

    let AnotherBox = Box::new(String::from("hello "));
    println!("Boxed string {}" , AnotherBox);
}
