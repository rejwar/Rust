struct Stack<T> {
    Data: Vec<T>,
}

impl<T> Stack<T> {
    fn New() -> Self {
        Stack { Data: Vec::new() }
    }

    fn Push(&mut self, Value: T) {
        self.Data.push(Value);
    }

    fn Pop(&mut self) -> Option<T> {
        self.Data.pop()
    }

    fn Peek(&self) -> Option<&T> {
        self.Data.last()
    }
}

fn main() {
    let mut MyStack = Stack::New();
    MyStack.Push(10);
    MyStack.Push(20);
    
    println!("Top Element: {:?}", MyStack.Peek());
    println!("Popped Element: {:?}", MyStack.Pop());
}
