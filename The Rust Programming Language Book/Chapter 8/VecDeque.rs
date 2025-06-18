use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Enqueue elements
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // Dequeue elements
    while let Some(value) = queue.pop_front() {
        println!("Dequeued: {}", value);
    }

    // Check if the queue is empty
    if queue.is_empty() {
        println!("The queue is empty.");
    } else {
        println!("The queue is not empty.");
    }
}
