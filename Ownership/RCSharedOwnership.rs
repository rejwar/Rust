use std::rc::Rc;

fn main() {
    let Original = Rc::new(String::from("Shared config"));
    println!("Count after create {}", Rc::strong_count(&Original));

    {
        let owner2 = Rc::clone(&Original);
        println!("Count inside scope {}", Rc::strong_count(&Original));
    }
    println!("Count at end {}", Rc::strong_count(&Original));
}
