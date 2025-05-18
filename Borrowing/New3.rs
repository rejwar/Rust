struct  Book{
    title:String,
    pages:u32,
}
fn PrintTitle (book : &Book) {
    println!("Book title : {}", book.title);
}

fn main () {
    let my_book = Book {
        title: String::from("Rust Journey"),
        pages: 200,
    };
    PrintTitle(&my_book);
}


// struct Counter  {
//     count: i32,
// }

// fn Increment(counter: &mut Counter) {
//     counter.count +=1;
// }

// fn main() {
//     let mut my_counter = Counter {count: 0};
//     Increment(&mut my_counter);
//     println!("Updated count {}", my_counter.count);
// }


fn FirstChar (s: &String ) -> &str {
    &s[..1]
}

fn main () {
    let name = String::from ("Rust");
    let ch = FirstChar(&name);
    println!("First character : {}", ch);
}
