// This program answers: "How to define a method with &mut self to modify struct data?"

struct Counter {
    value: i32,
}

impl Counter {
    fn Increment(&mut self) {
        // &mut self means we can change the struct's fields
        self.value += 1;
    }
}

fn main() {
    let mut counter = Counter { value: 0 };

    counter.Increment(); // value: 1
    counter.Increment(); // value: 2

    println!("Counter value: {}", counter.value);
}
