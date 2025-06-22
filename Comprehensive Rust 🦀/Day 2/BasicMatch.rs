fn MatchNumber(n: i32) {
    match n {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _=> println!("Something else "),
    }
}

fn main() {
    MatchNumber(10);
    MatchNumber(1);
}
