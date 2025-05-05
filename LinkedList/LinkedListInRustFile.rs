#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // লিস্টের শুরুতে একটি নতুন নোড যোগ করার ফাংশন
    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(), // বর্তমান head এর মালিকানা new_node.next এ মুভ করা হচ্ছে
        });
        self.head = Some(new_node);
    }

    // লিস্ট থেকে প্রথম নোডটি সরানোর ফাংশন
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // লিস্টের সমস্ত উপাদান প্রিন্ট করার ফাংশন
    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.data);
            current = &node.next;
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    println!("Empty list: {:?}", list);

    list.push(1);
    list.push(2);
    list.push(3);
    println!("List after push (3, 2, 1): {:?}", list);
    list.print_list(); // আউটপুট: 3, 2, 1

    println!("Popped: {:?}", list.pop()); // আউটপুট: Some(3)
    println!("List after pop: {:?}", list);
    list.print_list(); // আউটপুট: 2, 1
}
