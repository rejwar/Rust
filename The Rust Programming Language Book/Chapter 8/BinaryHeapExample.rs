// 

use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from([4, 1, 7, 3, 9]);

    while let Some(top) = heap.pop() {
        println!("Popped: {}", top);
    }
}
