struct Counter {
    Current: i32,
    Max: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.Current < self.Max {
            self.Current += 1;
            Some(self.Current)
        } else {
            None
        }
    }
}

fn main() {
    let MyCounter = Counter { Current: 0, Max: 5 };
    
    for Value in MyCounter {
        println!("Counter Value: {}", Value);
    }
}
