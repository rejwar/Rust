use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3, 4]);

    // Immutable borrow
    let reference = data.borrow();
    println!("Data: {:?}", *reference);
    drop(reference);

    // Mutable borrow
    {
        let mut mutable_reference = data.borrow_mut();
        mutable_reference.push(5);
    } // mutable_reference automatically dropped here

    println!("Updated data: {:?}", *data.borrow());

    // Multiple immutable borrows
    let reference1 = data.borrow();
    let reference2 = data.borrow();
    println!("Multiple immutable borrows: {:?} {:?}", *reference1, *reference2);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    fn send_message(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }

    fn message_count(&self) -> usize {
        self.sent_messages.borrow().len()
    }
}
