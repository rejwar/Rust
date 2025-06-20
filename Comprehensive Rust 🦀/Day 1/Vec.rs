fn generate_fib(n: usize) -> Vec<u32> {
    let mut seq = vec![0,1];
    for i in 2..n {
        let next = seq[i - 1] + seq[i - 2];
        seq.push(next);
    }
    seq
}

fn main() {
    println!("Fibonacci sequence up to 10 terms:");
    let fib_sequence = generate_fib(10);
    println!("{:?}", generate_fib(10));
}
