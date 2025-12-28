use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// fn main() {
//     let a = Rc::new(10);

//     let b = Rc::clone(&a);

//     thread::spawn(move || {
//         println!(" {}", b);
//     })
//     .join()
//     .unwrap();
// }
fn main() {
    let a = Arc::new(10);
    let b = Arc::clone(&a);

    thread::spawn(move || {
        println!("{}", b);
    })
    .join()
    .unwrap();
}
