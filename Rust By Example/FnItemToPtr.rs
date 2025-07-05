fn SayHello() {
    println!("Hello");
}

fn callFn(f: fn()) {
    f();
}

fn main() {
    callFn(SayHello);
}
