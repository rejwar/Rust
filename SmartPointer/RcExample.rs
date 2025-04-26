use std::rc::Rc;

fn main () {
    let Data = Rc::new(String::from("Hello World"));
    println!("Referencce count: {}", Rc::strong_count(&Data));

    let Data2 = Rc::clone(&Data);
    println!("Reference count after clone: {}", Rc::strong_count(&Data));
    {
        let Data3= Rc::clone(&Data);
        println!("Reference count in inner scope {}", Rc::strong_count(&Data));
    }

    println!("Reference Counter after inner scope : {}", Rc::strong_count(&Data));

    println!("Data: {}" , *Data);
    println!("Data2 : {}", *Data2);

    let Leaf = Rc::new(Node{
        Value: 3,
        Children: vec![],
    });

    let Branch = Rc::new(Node{
        Value : 5,
        Children: vec![Rc::clone(&Leaf)],
    });

    println!("Leaf reference count: {}" , Rc::strong_count(&Leaf));
    println!("Branch reference count :{}" , Rc::strong_count(&Branch));
}

struct Node {

    Value: i32,
    Children :Vec<Rc<Node>>, 
}
