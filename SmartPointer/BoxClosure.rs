fn UseBoxClosure() {
    let f: Box<dyn Fn()> = Box::new(|| {
        println!("Hello from Clousre in the Box");
    });
    f();
}

fn main() {
    UseBoxClosure();
}
