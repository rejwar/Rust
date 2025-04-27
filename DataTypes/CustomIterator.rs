use core::num;

struct  Counter {
    count: usize,
    max: usize,
}

impl  Iterator  for  Counter {

    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max{
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
    
}

fn main() {
    let counter = Counter { count: 0, max:5};

    for num in counter {
        println!("Number: {}" ,num);
    }
}
