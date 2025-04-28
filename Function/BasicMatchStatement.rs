fn MatchExample(x: i32 ) {
    match x {
        1 => println!("One"),
        2 => println!("two"),
        3 => println!("Three"),
        _=> println!("Something else"),

    }
}

fn main() {
    MatchExample(1);
    MatchExample(2);
}
