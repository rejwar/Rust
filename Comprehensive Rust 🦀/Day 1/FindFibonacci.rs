fn FindFibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
}

fn main() {
    let n = 10; // Example input
    let result = FindFibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
