enum List {
    Node(i32, Box<List>),
    Empty,
}

fn main() {
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Empty))));
}
