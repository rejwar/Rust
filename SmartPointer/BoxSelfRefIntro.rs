use core::slice;

struct Holder<'a> {
    text: String,
    part: &'a str,
}

fn UseBoxSelfRefINtro() {
    let mut boxed = Box::new(Holder {
        text: String::from("Hello world"),
        part: "",
    });

    let slice = &boxed.text[0..5];
    boxed.part = slice;

    println!("Full {}", boxed.text);
    println!("Part ={}", boxed.part);
}

fn main() {
    UseBoxSelfRefINtro();
}
