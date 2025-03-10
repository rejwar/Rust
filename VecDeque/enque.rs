use std::collections::VecDeque;

impl <T> Queue<T> {
    fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    fn enqueue(&mut self , value: T) {
        self.data.push_back(value)
    }
}