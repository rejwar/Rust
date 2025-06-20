fn main() {
    let age:i32 = 25;

    let has_permission = true;

    if age >= 18 && has_permission {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }
}
