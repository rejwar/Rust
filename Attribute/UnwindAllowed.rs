#[unwind(allowed)]
fn RecoverableFunction() {
    println!("This function can safely unwind!");
}
