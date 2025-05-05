use std::collections::BinaryHeap;

fn main() {
    let mut PQ = BinaryHeap::new();

    PQ.push(30);
    PQ.push(10);
    PQ.push(20);

    println!("Highest Priority: {:?}", PQ.pop()); // ✅ Max Priority Element
}
