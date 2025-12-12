fn main() {
    let v = vec![String::from("A"), String::from("B")];
    print_slide(&v[..]);
}

fn print_slide(items: &[String]) {
    println!("First item {}", items[0]);
}
