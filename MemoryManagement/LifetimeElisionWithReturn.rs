fn main() {
    let greeting = String::from("Hi there!");
    println!("{}", echo(&greeting));
}

fn echo(input: &str) -> &str {
    input
}
