use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(3);

    // Pop elements in descending order
    while let Some(value) = heap.pop() {
        println!("{}", value);
    }
}
