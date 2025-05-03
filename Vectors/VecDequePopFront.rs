use std::collections::VecDeque;

fn main() {
    let mut Queue: VecDeque<i32> = VecDeque::front(vec![10,20,30]);
    Queue.pop_front();
    println!("Updated Queue : {:?}" , Queue);
}
