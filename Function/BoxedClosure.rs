fn CallFunction(f: Box<dyn Fn()>) {
    f();
}

fn main() {
    let shout = Box::new (|| println!("Dynamically Called"));
    CallFunction(shout);
}
