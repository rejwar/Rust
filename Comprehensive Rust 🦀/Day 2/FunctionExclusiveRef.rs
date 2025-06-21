// 
fn make_uppercase(s: &mut String) {
    s.make_ascii_uppercase();
}

fn main() {
    let mut name = String::from("md");
    make_uppercase(&mut name);
    println!("Uppercased: {}", name);
}
