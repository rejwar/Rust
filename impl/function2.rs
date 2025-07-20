struct Note<'a> {
    text: &'a str ,
}

impl <'a> Note <'a>  {
    fn Show(&self) {
        println!("Note {}", self.text);
    }
}