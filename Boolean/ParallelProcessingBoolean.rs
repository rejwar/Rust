use std::thread;

fn main() {
    let TaskOne: bool = true;
    let TaskTwo: bool = false;

    let HandleOne = thread::spawn(move || {
        println!("Task One Completed: {}", TaskOne);
        TaskOne
    });

    let HandleTwo = thread::spawn(move || {
        println!("Task Two Completed: {}", TaskTwo);
        TaskTwo
    });

    let ResultOne: bool = HandleOne.join().unwrap();
    let ResultTwo: bool = HandleTwo.join().unwrap();

    println!("All Tasks Successful: {}", ResultOne && ResultTwo);
}
