fn main() {
    let command = Some(String::from("Shutdown"));

    if let Some(ref c) = command 
    {
        if c == "hutdown"{
            println!("System shutting down");
    } else {
        println!("Command Ignored");
    }
}
}