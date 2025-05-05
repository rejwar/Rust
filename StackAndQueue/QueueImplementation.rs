use std::collections::VecDeque;

struct Queue<T> {
    Data: VecDeque<T>,
}

impl<T> Queue<T> {
    fn New() -> Self {
        Queue { Data: VecDeque::new() }
    }

    fn Enqueue(&mut self, Value: T) {
        self.Data.push_back(Value);
    }

    fn Dequeue(&mut self) -> Option<T> {
        self.Data.pop_front()
    }

    fn Front(&self) -> Option<&T> {
        self.Data.front()
    }
}

fn main() {
    let mut MyQueue = Queue::New();
    MyQueue.Enqueue(1);
    MyQueue.Enqueue(2);

    println!("Front Element: {:?}", MyQueue.Front());
    println!("Dequeued Element: {:?}", MyQueue.Dequeue());
}
