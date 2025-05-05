struct CircularQueue {
    Data: Vec<i32>,
    Front: usize,
    Rear: usize,
    Size: usize,
}

impl CircularQueue {
    fn New(Size: usize) -> Self {
        CircularQueue { Data: vec![0; Size], Front: 0, Rear: 0, Size }
    }

    fn Enqueue(&mut self, Value: i32) -> bool {
        if (self.Rear + 1) % self.Size == self.Front {
            return false; // Queue Full
        }
        self.Data[self.Rear] = Value;
        self.Rear = (self.Rear + 1) % self.Size;
        true
    }

    fn Dequeue(&mut self) -> Option<i32> {
        if self.Front == self.Rear {
            return None; // Queue Empty
        }
        let Value = self.Data[self.Front];
        self.Front = (self.Front + 1) % self.Size;
        Some(Value)
    }
}

fn main() {
    let mut MyQueue = CircularQueue::New(5);

    MyQueue.Enqueue(10);
    MyQueue.Enqueue(20);
    
    println!("Dequeued: {:?}", MyQueue.Dequeue());
}
