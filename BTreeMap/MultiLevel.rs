use core::task;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive( Debug, Eq , PartialEq)]



struct Task{
    priority: i32,
    deadline: i32,
}

fn main() {
    let mut tasks: BTreeMap<Task , String> = BTreeMap::new();
    tasks.insert(Task { priority: 1, deadline: 5 }, "Fnisnished Rust home work". to_string());
    tasks.insert(Task { priority: 2, deadline: 6 }, "Write blog".to_string());

    for (task ,desc ) in &tasks {
        println!("Priority {} Deadline is {} Tasks {}", task.priority ,task.deadline , desc);
    }
}