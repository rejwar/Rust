use std::fmt::Display;

struct Wrapper<T> {
    value : T,
}

impl <T : Display> Wrapper<T> {
    fn Show(&self) {
        println!("Value : {}", self.value);
    }
}

fn main() {
    let w1 = Wrapper {value: 123};
    let w2 = Wrapper { value: "Hello"};

    w1.Show();
    w2.Show();
}