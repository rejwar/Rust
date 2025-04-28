fn MultiplePatternMatchExample(x: i32) {
    match x {
        1 | 3 | 5 | 7 | 9 => println!("Odd single digit"),
        2 | 4 | 6 | 8  => println!("Even single Digit"),
        _=> println!("Other Number"),
    }
}

fn main() {
    MultiplePatternMatchExample(4);
}
