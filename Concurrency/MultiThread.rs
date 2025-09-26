use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            let start = i * 10 +1;
            let end : i32 = (i + 1) * 10;
            let sum : i32 = (start..=end).sum();
            println!("Thread is {} calcuated s um of {} {} {}" , i , start , end , sum);
            sum
        });
        handles.push(handle);
    }

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
}