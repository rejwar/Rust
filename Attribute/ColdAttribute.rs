#[cold]
fn HandleError() {
    println!("This function is rarely executed.");
}

fn main() {
    HandleError();
}
