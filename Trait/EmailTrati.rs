trait Message {
    fn send_message(&self) -> String;
}

struct Email {
    to: String,
}

impl Message for Email {
    fn send_message(&self) -> String {
        format!("Sending Message to {}", self.to)
    }
}

fn main() {}
