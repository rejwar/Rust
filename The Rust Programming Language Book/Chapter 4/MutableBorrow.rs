fn AppendWorld(Text: &mut String) {
    Text.push_str(" World");
}

fn main() {
    let mut Greeting = String::from("Hello");
    AppendWorld(&mut Greeting);
    println!("{}", Greeting);
}
