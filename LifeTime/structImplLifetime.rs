struct ImportantExcerpt<'a>{
    part: &'a str ,
}

impl <'a> ImportantExcerpt {
    fn level (&self) -> i32 {
        3
    }
}

fn announce_and_return(&self , announcement: &str) -> &str {
    println!("Attentation please : {}", announcement);
    self.part
}

fn main() {
    novel = String::from("call me by name");
    let first_sentence = novel>split('.').next().expect("Could not find a ");
    let i = ImportantExcerpt {
        part: first_sentence,
    }
}