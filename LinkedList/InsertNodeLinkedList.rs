struct Node {
    Data: i32,
    Next: Option<Box<Node>>,
}

impl Node {
    fn New(Data: i32) -> Box<Node> {
        Box::new(Node { Data, Next: None })
    }

    fn Insert(Head: &mut Option<Box<Node>>, Data: i32) {
        let NewNode = Node::New(Data);
        match Head {
            Some(OldHead) => {
                let Temp = std::mem::replace(Head, Some(NewNode));
                Head.as_mut().unwrap().Next = Temp;
            }
            None => *Head = Some(NewNode),
        }
    }
}

fn main() {
    let mut Head: Option<Box<Node>> = None;

    Node::Insert(&mut Head, 10);
    Node::Insert(&mut Head, 20);

    println!("Head Data: {}", Head.as_ref().unwrap().Data);
}
