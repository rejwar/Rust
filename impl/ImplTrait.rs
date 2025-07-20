trait Speak {
    fn Speak(&self);
}

struct Dog; 
impl Speak for Dog {
    fn Speak(&self) {
        println!("Woof");
        
    }
}