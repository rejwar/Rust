fn main() {
    let is_handsome: bool = true;

    let is_silly: bool = false;
    println!("Is he handsome? {}", is_handsome);

    let age: i32 = 18;
    let is_young: bool = age < 35;
    println!("Is he young? {}", is_young);
    println!("{} is true", is_young);

    println!("{} {}" , age.is_positive(), age.is_negative());
}
