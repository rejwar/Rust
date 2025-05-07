#[stack_protector]
fn SecureFunction() {
    println!("Protected against buffer overflow attacks!");
}
