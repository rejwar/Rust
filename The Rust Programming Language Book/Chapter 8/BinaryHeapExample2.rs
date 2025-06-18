use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from([1,2,3,4,5]);

    while let Some(value) = heap.pop() {
        println!("{}", value);
    }
}
