fn main() {
    let fullname = "kannan , sudhakaran , Tutorials ";

    for token in fullname.split(",") {
        println!("token is {}", token );
    }
    println!("\n");
    let tokens:Vec<&str> = fullname.split(",").collect();
    println!("first is {}",tokens[0]);
    println!("lastname is {}",tokens[1]);
    println!("company is {}",tokens[2]);
}
