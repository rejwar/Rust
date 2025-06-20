fn main() {
    let number = Some(5);

    match  number {
        Some(n) => {
            println!("The number is: {}", n);
            n*2
        },
        None => 0,
        };
    }
