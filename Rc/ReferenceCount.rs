use std::rc::Rc;

fn main() {
    let Shared = Rc::new("Hello Rust!");
    let _Clone1 = Rc::clone(&Shared);
    let _Clone2 = Rc::clone(&Shared);

    println!("Reference Count: {}", Rc::strong_count(&Shared));
}
