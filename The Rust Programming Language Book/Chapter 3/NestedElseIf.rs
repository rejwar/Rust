fn classify_number(x: i32) {
    if x % 2 == 0 {
        if x > 0 {
            println!("Positive even number");
        } else {
            println!("Negative even number");
        }
    } else if x > 0 {
        println!("Positive odd number");
    } else {
        println!("Negative odd number");
    }
}

fn main() {
    classify_number(6);
    classify_number(-3);
}
