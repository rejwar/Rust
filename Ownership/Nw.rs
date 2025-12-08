fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let greet = || println!("Hi");
    apply(greet);
}
