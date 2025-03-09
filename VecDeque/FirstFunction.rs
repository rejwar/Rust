use std::collections::VecDeque;

impl <T> Queue<T> {
    fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }
}