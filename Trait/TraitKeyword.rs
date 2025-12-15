trait Message {
    fn send_message(&self) -> String;

    fn log_status(&self) {
        println!("Status: Message sent successfully");
    }
}
