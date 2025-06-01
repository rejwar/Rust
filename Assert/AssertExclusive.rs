use std::ops::Range;

fn main() {
    let ExclusiveRange = Range { start: 1, end: 5 };
    assert_eq!((1..5), ExclusiveRange);
    println!("Exclusive Range: {:?}", ExclusiveRange);
}
