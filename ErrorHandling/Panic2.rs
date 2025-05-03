fn CheckPositive(Number: i32) {
    if Number < 0 {
        panic!("Negative number found!");
    }

    println!("Valid Number {}" , Number);
}

fn main() {
    CheckPositive(10);
    CheckPositive(-5);
}
