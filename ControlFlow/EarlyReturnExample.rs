fn main() {
    if check_condition() {
        return;
    }
    println!("Condition not met");
}

fn check_condition() -> bool {
    true
}
