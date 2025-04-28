fn PrintNumber() {
    if n == 0{
        return;
    }

    println!("Number :{}",n);
    PrintNumber (n-1);
}

fn main() {
    PrintNumber (5);
}
