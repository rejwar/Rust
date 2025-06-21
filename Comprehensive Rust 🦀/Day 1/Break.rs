fn BreakExample() {
    for i in 1..=10 {
        if i > 5 {
            println!("Reached the limit , breaking limit");
            break;
        }
        println!("Current number is {}", i);
    }

    println!("Loop finished");
}
