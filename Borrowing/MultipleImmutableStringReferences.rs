fn main() {
    let car:String = String::from("Red");
    let ref1: &String = &car;
    let ref2: &String = &car;
    println!("{} and {} {}", ref1 ,ref2 ,&car);
}
