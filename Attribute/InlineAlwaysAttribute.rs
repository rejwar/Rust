#[inline(always)]
fn FastCompute(x: i32) -> i32 {
    x * x + x
}

fn main() {
    println!("Result: {}", FastCompute(5));
}
