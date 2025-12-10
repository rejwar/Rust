fn main() {
    let text = String::from("Hello");

    let print_closure = || {
        println!("THe text is {}", text);
    };

    print_closure();

    println!("Main is still {}", text);
}
