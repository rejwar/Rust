fn fib(n: u32) -> u32 {
    let (mut a , mut b ) = (0, 1);
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp; 
    }
    a
}

fn main() {
    println!("Fibonacci of 10 is: {}", fib(10));
}
