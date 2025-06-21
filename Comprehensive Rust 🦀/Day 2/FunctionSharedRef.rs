fn print_length(s: &String) {
    println!("Length : {}", s.len());

}

fn main() {
    let text  = String::from("hello");
    print_length(&text);
    println!("Still unable :{}", text);
}
