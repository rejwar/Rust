use std::option::Option;

struct Node<T> {
    Data: T,
    Next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    Top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn New() -> Self {
        Stack { Top: None }
    }

    fn Push(&mut self, Value: T) {
        let NewNode = Box::new(Node { Data: Value, Next: self.Top.take() });
        self.Top = Some(NewNode);
    }

    fn Pop(&mut self) -> Option<T> {
        self.Top.take().map(|Node| {
            self.Top = Node.Next;
            Node.Data
        })
    }
}

fn main() {
    let mut MyStack = Stack::New();
    MyStack.Push(100);
    MyStack.Push(200);

    println!("Popped: {:?}", MyStack.Pop());
}
