// ❓ Question: How do you define a static variable in Rust?

struct MyType;

impl MyType {
    fn new() -> Self {
        println!("MyType initialized");
        MyType
    }
}

static X: MyType = MyType::new(); // Compile-time constant initialization

fn main() {
    println!("Accessing static X...");
    let _ = &X;
}
