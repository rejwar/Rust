fn main() {
    let mut Data: String = String::from("Double Trouble ");
    let Ref1: &mut String = &mut Data;
    let Ref2: &mut String = &mut Data;

    println!("{} {}", Ref1 , Ref2);
}