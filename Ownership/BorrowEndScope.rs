fn main() {
    let mut Data: String = String::from("Scope Check");
    let Ref1: &String = &Data;
    println!("Ref1 {}" , Ref1);
    let Ref2: &mut String = &mut Data;
    Ref2.push_str("Updated");
    println!("Ref2 {}", Ref2);
    
}