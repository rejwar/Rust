unsafe fn dangerous_function() {
    println!("This is an unsafe function");
}

fn main() {
    unsafe  {
        dangerous_function();
    }
}
