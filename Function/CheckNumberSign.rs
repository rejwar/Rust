fn CheckNumber(n:i32) -> &'static str {
    if n > 0 {
        return "positive";
    } else if n < 0 {
        return "Negative";
    }

    "Zero"
}

fn main() {
    println!("{}" , CheckNumber(-9));
    println!("{}" , CheckNumber(-3));
    println!("{}" , CheckNumber(0));
}
