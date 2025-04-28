fn Factorail (n: u64) -> u64 {
    if n==0 {
        1
    } else {
        n*Factorail(n-1)
    }
}
fn main() {
    Factorail(5);
    println!("Factorial of 5 is {}" , Factorail(5));
}
