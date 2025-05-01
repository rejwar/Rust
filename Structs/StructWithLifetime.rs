struct  Document<'a> {
    Content : &'a str,
}

fn main() {
    let Text: String = String::from("Rust lifetime Example");
    let Doc: Document = Document { Content: &Text};

    println!("{}" , Doc.Content);
}
