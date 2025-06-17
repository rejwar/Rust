// Question: How does Rust's built-in `Option` enum work?

fn UseOptionEnum() {
    let some_value = Some(100);
    let no_value: Option<i32> = None;

    match some_value {
        Some(num) => println!("Got number: {}", num),
        None => println!("No value found"),
    }

    match no_value {
        Some(num) => println!("Won't print"),
        None => println!("Correct: nothing inside"),
    }
}
