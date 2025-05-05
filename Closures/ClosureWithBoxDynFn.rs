fn CreateClosure() -> Box<dyn Fn()> {
    Box::new(|| println!("Closure inside Box"))
}

fn main() {
    let DynamicClosure = CreateClosure();
    DynamicClosure();
}
