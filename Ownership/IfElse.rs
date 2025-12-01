fn main() {
    let x = 2;

    match x {
        1 => println!("[translate:one]"),
        2 => println!("[translate:two]"),
        3 => println!("[translate:three]"),
        _ => println!("[translate:something else]"),
    }
}
