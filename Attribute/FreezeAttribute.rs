#[freeze]
static MY_CONFIG: i32 = 100;

fn main() {
    println!("Configuration: {}", MY_CONFIG);
    // ❌ Compiler Error: Cannot modify frozen data!
}
