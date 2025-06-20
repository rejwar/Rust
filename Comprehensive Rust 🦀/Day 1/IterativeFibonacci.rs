fn mfibonacchi(n: i32 ) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 2;

    for _ in 3..=n {
        let next = a + b + c;
        a = b;
        b = c;
        c = next;
    }

    c
}


fn main() {
    let n = 10; // Example input
    let result = mfibonacchi(n);
    println!("The {}-th modified Fibonacci number is: {}", n, result);
}
