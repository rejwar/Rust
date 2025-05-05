struct Node {
    Data: i32,
    Next: Option<Box<Node>>,
}

fn PrintList(Head: &Option<Box<Node>>) {
    let mut Current = Head;
    while let Some(Node) = Current {
        println!("Node Data: {}", Node.Data);
        Current = &Node.Next;
    }
}

fn main() {
    let Node3 = Box::new(Node { Data: 30, Next: None });
    let Node2 = Box::new(Node { Data: 20, Next: Some(Node3) });
    let Node1 = Box::new(Node { Data: 10, Next: Some(Node2) });

    PrintList(&Some(Node1));
}
