fn RangeMatchExample (score: u32) {
    match score {
        0..=49 => println!("Fail"),
        50..=69 => println!("Pass"),
        70..=89 => println!("Good"),
        90..=100 => println!("Excellentn"),
        _=> println!("Something else"),
    }
}

fn main() {
    RangeMatchExample(70);
    RangeMatchExample(999);
}
