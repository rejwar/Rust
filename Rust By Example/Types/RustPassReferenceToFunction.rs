fn print_value(x: &i32) {
    println!("Value {}",x);
}

fn main() {
    let a = 42;
    let b = &a;
    print_value(b);
}
