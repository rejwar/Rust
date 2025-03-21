fn main() {
    let mut text = String::from("Hello");
    append_exclamation(&mut text);
    println!("{}", text);
}

fn append_exclamation<'a>(input: &'a mut String) {
    input.push('!');
}
