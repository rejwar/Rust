fn main() {
    let s = String::from("Hi");

    let s = secret_root(s);
    println!(" {}", s);
}

fn secret_root(s: String) -> String {
    s
}
