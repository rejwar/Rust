#[derive(Debug)]

struct BTree{
    root: Option<Box<Node>>,
    t: usize,
}

impl BTree {
    fn new(t: usize) -> Self {
        BTree { root: None, t }
    }
}