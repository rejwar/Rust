fn main() {
    let Data:String =String::from("Immutable Data");
    let Ref1: &String = &Data;
    let Ref2: &String = &Data;
    println!("{} and {}" , Ref1 , Ref2);
    println!("Owner is {}", Data);
}