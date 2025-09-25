// use std::thread;

// fn main() {
//     let handle = thread::spawn( || {
//         for i in 1..=5 {
// for i in 1..=5 {
//     println!("Thread 1 {}", i);
// }
//         }

//                     let sum: i32 = (1..=10).sum();
//             sum;
//     });
    

//     let  result = handle.join().unwrap();
//     println!(" Sum from thread is {:?}" , result);
// }


//wrong answer 


use std::thread;

use rayon::result;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread is 1 {}" ,i);
        }

        let sum: i32 = (1..=10).sum();
        sum
    });
    let result= handle.join().unwrap();
    println!("sum from thread is {}", result);
}