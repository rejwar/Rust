// How to use struct with lifetimes?

struct Message<'a> {
    content: &'a str,
}

fn UseStructWithLifetime() {
    let msg_text = String::from("Hello, Rust!");
    let msg = Message { content: &msg_text };
    println!("Message: {}", msg.content);
}
