// Question: How do you generate and print the Collatz sequence for a given positive integer?

fn GenerateCollatzSequence(mut n: u64) {
    if n == 0 {
        println!("Input must be a positive integer greater than 0.");
        return;
    }

    print!("Collatz sequence for {}: ", n);
    while n != 1 {
        print!("{} → ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("1");
}

fn main() {
    GenerateCollatzSequence(19);
}
