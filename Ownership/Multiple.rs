fn main() {
    let x = 2;

    match x {
        1 | 2 => println!("One or two"),
        _ => println!("Something else"),
    }
}
