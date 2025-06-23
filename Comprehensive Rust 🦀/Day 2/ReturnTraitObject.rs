
fn CreateDisplayable(value: i32) -> Box<dyn std::fmt::Display> {
    Box::new(value)
}

fn main() {
    let displayable = CreateDisplayable(100);
    println!("{}", displayable); // 100
}
