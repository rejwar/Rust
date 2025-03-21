enum Task {
    Pending,
    Completed,
}

enum Priority {
    High,
    Medium,
    Low,
}

fn main() {
    let task = Task::Completed;
    let priority = Priority::High;

    match task {
        Task::Pending => println!("Task is pending."),
        Task::Completed => println!("Task is completed."),
    }

    match priority {
        Priority::High => println!("High priority."),
        Priority::Medium => println!("Medium priority."),
        Priority::Low => println!("Low priority."),
    }
}
