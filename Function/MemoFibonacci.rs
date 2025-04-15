use std::result;

fn Fibonacci(n:usize, memo:&mut Vec<Option <u64>>) -> u64{
    if let Some(val) = memo[n] {
        return val;
    }

    let result = if n<=1 {
        n as u64 
    }else {
        Fibonacci(n-1, memo) + Fibonacci(n-2, memo)
    };

    memo[n] = Some(result);
    result
}

fn main() {
    let n =20;
    let mut memo = vec![None; n+1];
    println!("Fib ({}) = {}" , n, Fibonacci(n, &mut memo));
}
