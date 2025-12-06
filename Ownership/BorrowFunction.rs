// fn len(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("Nwe ");
//     let l = len(&s);
//     println!("Length = {}", l);
// }
fn len(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("Hello");
    let l = len(&s);
    println!(" Length {}", l);
}
