fn main() {
    let example_string = String::from("Example_string");
    print_literal(example_string.as_str());
}

fn print_literal(data: &str) {
    println!("displaying string literal {}" , data);
}
