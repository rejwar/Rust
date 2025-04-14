struct Note<'a> {
    content: &'a str,
}

fn main() {
    let msg = String::from("hello");
    let note = Note {content: &msg};
    println!("{}" ,note.content);
}
