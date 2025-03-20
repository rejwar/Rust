fn main() {
    let mut text = String::from("Mutable");
    let reference = get_mutable_reference(&mut text);
    reference.push_str(" Reference");
    println!("{}", text);
}

fn get_mutable_reference(input: &mut String) -> &mut String {
    input
}
