use std::rc::Rc;

#[derive(Debug)]

struct  Node {
    value: i32,
    children : Vec<Rc<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {value : 1 , children : vec![]});

    let branch1 = Rc:: new(Node {value : 2 , children : vec![Rc::clone(&leaf)]});
    let branch2 = Rc::new(Node { value: 3, children: vec![Rc::clone(&leaf)] });

    println!("Leaf ref counted : {}", Rc::strong_count(&leaf));

}
