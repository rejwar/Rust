fn main() {
    let mut Data: String = String::from("Fn Conflice ");
    UseImmutable(&Data);
    UseMutable(&Data);
}

fn UseImmutable(Input: &String) {
    println!("Immutable {}", Input);
} 

fn UseMutable (Input: &String) {
    Input.push_str("Changed");
    println!("Mutable {}", Input);
}