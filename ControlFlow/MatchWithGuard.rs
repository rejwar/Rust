fn MatchGuardExample(x:i32){
match x {
    n if n% 2 == 0 => println!("Even Number"),
    n if n % 2 !=0 => println!("Odd Number"),

    _=> println!("Unknown"),
}
}

fn main() {
    MatchGuardExample(7);
}
