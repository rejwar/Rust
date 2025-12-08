// fn main() {
//     let s1 = String::from("Hello");

//     let len = calculate_length(&s1);

//     println!(" The length of '{}' {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!(" The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
