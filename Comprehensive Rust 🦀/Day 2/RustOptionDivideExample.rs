fn Divide(Num1: f64, Num2: f64) -> Option<f64> {
    if Num2 == 0.0 {
        None // ভাগশূন্য হলে None রিটার্ন করে
    } else {
        Some(Num1 / Num2) // ভাগফল রিটার্ন করে
    }
}

fn main() {
    let Result = Divide(10.0, 2.0);

    match Result {
        Some(Value) => println!("Result: {}", Value), // যদি ভ্যালু থাকে
        None => println!("Cannot divide by zero!"),   // যদি ভ্যালু না থাকে
    }
}
