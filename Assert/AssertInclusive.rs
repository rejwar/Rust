use std::ops::RangeInclusive;
fn main() {
    let InclusiveRange = RangeInclusive::new(1, 5);
    assert_eq!((1..=5), InclusiveRange);
    println!("Inclusive Range: {:?}", InclusiveRange);
}
