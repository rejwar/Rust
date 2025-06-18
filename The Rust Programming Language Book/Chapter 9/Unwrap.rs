// Question: How can unwrap cause panic?

fn UnwrapPanicExample() {
    let maybe_name: Option<&str> = None;

    // This will panic because the Option is None
    println!("Name: {}", maybe_name.unwrap());
}
fn main() {
    UnwrapPanicExample();
}
