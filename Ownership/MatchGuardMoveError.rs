fn main() {
    let x = Some(String::from("Hello"));

    match x {
        Some(s) if s.len() > 0 => {
            println!("Metched {}", s);
        }

        _ => println!("Other"),
    }
}
