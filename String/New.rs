fn main() {
    let example_string = String::from("enteral_string");
    print_literal(example_string.as_str());
}

fn print_literal (data: &str){
    println!("Displaying string literal {}" , data);
}
