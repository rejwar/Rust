use std::collections::VecDeque;

struct Queue<T> {
    data: VecDeque<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        Queue { data: VecDeque::new(), }
    }
}

fn enqueue(&mut self , value: T) {
    self.data.push_back(value);
}

fn main() {
    let mut   q = Queue::new();
    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);
    println!("Queue inner side {:?}", q.data);
}