// Question: How do we use panic! conditionally?

fn ConditionalPanicExample(x: i32) {
    if x == 0 {
        panic!("Zero is not allowed!");
    } else {
        println!("All good: {}", x);
    }
}
fn main() {
    ConditionalPanicExample(5);
}
