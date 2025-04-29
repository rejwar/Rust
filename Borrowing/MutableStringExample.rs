fn main() {
    let mut greeeting: String = String::from("Hello");
    println!("Greeting: {}", greeeting);


    greeeting.push_str("World !");
    println!("Updated greeting : {}", greeeting);

    let name: String = "Alice"  . to_string();
    println!("Name: {}", name);

}
