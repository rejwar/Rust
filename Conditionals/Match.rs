fn check_numbers(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

fn main() {
    check_numbers(1);
    check_numbers(2);
    check_numbers(3);
    check_numbers(4);
}
