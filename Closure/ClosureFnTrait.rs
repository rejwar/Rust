fn Apply<F : Fn()> (f: F ) {
    f();
}

fn main() {
    let Hello = || println!("FN trait closure ");
    Apply(Hello);
}