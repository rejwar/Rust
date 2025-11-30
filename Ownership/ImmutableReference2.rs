fn main() {
    let s = String::from("hello");

    let len = length(&s);

    println!("len = {}", len);
    println!("s = {}", s);
}

fn length(s_ref: &String) -> usize {
    s_ref.len()
}
