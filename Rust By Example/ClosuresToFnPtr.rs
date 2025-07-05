fn call_fn(f: fn(i32)-> i32) {
    println!("Result {}", f(5));
}

fn main() {
    let add_one = |x| x+1;
    call_fn(add_one);
}
