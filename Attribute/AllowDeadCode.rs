#[allow(dead_code)]
fn UnusedFunction() {
    println!("This function is not used but won't trigger a warning!");
}

fn main() {
    println!("No warnings for unused functions!");
}
