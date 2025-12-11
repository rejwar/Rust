fn main() {
    let text = String::from("No move");

    let print_it = || {
        println!("{}", text);
    };

    print_it();
    println!("Text still here {}", text);
}
