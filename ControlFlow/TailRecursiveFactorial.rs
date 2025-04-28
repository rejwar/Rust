fn TailFactorial(n: u64 , acc:u64) -> u64 {
    if n == 0 {
        acc
    } else {
        TailFactorial(n-1, acc*n)
    }
}

fn main() {
    println!("Tail Factorial of  7 is {}", TailFactorial(7, 1))
}
