fn Apply<F: Fn(i32) -> i32>(f:F , val:i32) -> i32 {
    f(val)
}
fn main() {
    let Result = Apply(|x|x +10,5);
    println!("Result : {}", Result);
}
