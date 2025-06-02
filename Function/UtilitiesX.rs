mod utilities{
    pub fn print_message() {
        println!("Hello from the utilities module!");
    }
}

fn main() {
    utilities::print_message();
    println!("This is the main function.");
}
