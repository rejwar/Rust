struct Node {
    Data: i32,
    Next: Option<Box<Node>>,
}

fn Remove(Head: &mut Option<Box<Node>>) {
    if let Some(OldHead) = Head.take() {
        *Head = OldHead.Next;
    }
}

fn main() {
    let mut Head = Some(Box::new(Node { Data: 10, Next: None }));
    Remove(&mut Head);
    
    match Head {
        Some(Node) => println!("New Head Data: {}", Node.Data),
        None => println!("List is empty!"),
    }
}
