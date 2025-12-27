fn UseBoxLeak() {
    let bx = Box::new(String::from("I never get freed"));
    let static_ref: &'static str = Box::leak(bx);
    println!("{}", static_ref);
}

fn main() {
    UseBoxLeak();
}
