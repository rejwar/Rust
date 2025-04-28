fn AdvancedMatchExample(number: i32 ) {
    match number{
        1..=5 =>  println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _=> println!("Out of range"),
    }
}

fn main() {
    

    AdvancedMatchExample(5);
    AdvancedMatchExample(6);
    AdvancedMatchExample(7);
}
