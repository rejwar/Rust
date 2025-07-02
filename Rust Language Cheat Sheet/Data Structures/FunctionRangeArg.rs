fn SumRange(range: std::ops::Range<u32>) -> u32 {
    range.sum()
}

fn main() {
    let total = SumRange(1..5);
    println!("Sum is {}", total);
}
