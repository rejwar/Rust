fn PrintFibonacci(limit: u32) {
    let mut a = 0;
    let mut b = 1;

    while a <= limit {
        println!("{}", a);
        let next = a + b;
        a = b;
        b = next;
    }
}

fn main() {
    let limit = 100; // You can change this limit as needed
    PrintFibonacci(limit);
}
