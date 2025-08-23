fn CalcualteLength(inputString: &String) -> usize  {
    inputString.len()
}

fn ModifyString(text: &mut String) {
    text.push_str("World");
}

fn main() {
    let greeting = String::from("Hello");

    let length = CalcualteLength(&greeting);
    println!("Length {}", length);

    ModifyString(&mut greeting);
    println!("Modified {}", greeting);
}