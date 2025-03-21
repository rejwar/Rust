fn main() {
    let x = String::from("Rust");
    let y = String::from("Programming");
    // let result = conflicting_lifetimes(&x, &y); // Uncomment to see error
}

fn conflicting_lifetimes<'a, 'b>(_x: &'a str, _y: &'b str) -> &'a str {
    _x
}
