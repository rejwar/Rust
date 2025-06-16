// Question: How does the `Option` enum work in Rust for handling null-like values?

fn UseOptionEnum() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(val) => println!("The number is: {}", val),
        None => println!("No number found."),
    }

    match no_number {
        Some(val) => println!("This will not print"),
        None => println!("Correct: No value"),
    }
}
