trait SayIt {
    fn speak(&self);
}

struct Dog;
impl SayIt for Dog {
    fn speak(&self) {
        println!("Woolf");
    }
}
fn main() {
    let d = Dog;
    d.speak();
}
