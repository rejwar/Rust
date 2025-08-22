fn main() {
    let mut Data: String = String::from("Confilict Case");
    {
    let Ref1: &String = &Data;
    println!("Read :{}", Ref1);

    }
    let Ref2: &mut String =  &mut Data;
    Ref2.push_str("Upadeted");
    println!("{} " Ref2);
}