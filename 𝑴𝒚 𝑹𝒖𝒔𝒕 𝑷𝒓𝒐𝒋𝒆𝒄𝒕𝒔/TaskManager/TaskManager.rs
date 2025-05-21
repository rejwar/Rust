// FileName: TaskManager.rs

// Note: Standard Rust convention uses snake_case for function and variable names.
// However, PascalCase is used here as per specific user instruction ("Use pascal Case all the time").

use std::io::{self, Write}; // For input/output operations
use std::collections::HashMap; // To store tasks efficiently by ID

/// Enum representing the status of a task.
#[derive(Debug, Clone, PartialEq)] // PartialEq allows for direct comparison of statuses
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

/// Struct representing a single task.
#[derive(Debug, Clone)]
struct Task {
    Id: u32,
    Description: String,
    Status: TaskStatus,
    // Example of an optional field:
    // Priority: Option<u8>,
}

/// Struct for managing a collection of tasks.
#[derive(Debug)]
struct TaskManager {
    Tasks: HashMap<u32, Task>, // Using HashMap for quick task retrieval by Id
    NextId: u32,               // To generate unique IDs for new tasks
}

impl TaskManager {
    /// Creates a new instance of TaskManager.
    fn New() -> Self {
        TaskManager {
            Tasks: HashMap::new(),
            NextId: 1,
        }
    }

    /// Adds a new task to the manager.
    fn AddTask(&mut self, Description: String) {
        let NewTask = Task {
            Id: self.NextId,
            Description,
            Status: TaskStatus::Pending,
            // Priority: None, // If Priority field was added
        };
        self.Tasks.insert(self.NextId, NewTask);
        println!("✅ Task Added Successfully With ID: {}", self.NextId);
        self.NextId += 1; // Increment ID for the next task
    }

    /// Retrieves and displays a task by its ID.
    fn ViewTask(&self, Id: u32) {
        match self.Tasks.get(&Id) {
            Some(FoundTask) => {
                println!("\n--- Task Details ---");
                println!("🆔 ID: {}", FoundTask.Id);
                println!("📝 Description: {}", FoundTask.Description);
                println!("📊 Status: {:?}", FoundTask.Status);
                // if let Some(Priority) = FoundTask.Priority {
                //     println!("⭐ Priority: {}", Priority);
                // }
                println!("--------------------");
            }
            None => println!("⚠️ Task With ID {} Not Found.", Id),
        }
    }

    /// Updates the status of an existing task.
    fn UpdateTaskStatus(&mut self, Id: u32, NewStatus: TaskStatus) {
        if let Some(TaskToUpdate) = self.Tasks.get_mut(&Id) {
            TaskToUpdate.Status = NewStatus;
            println!("🔄 Task {} Status Updated Successfully.", Id);
        } else {
            println!("⚠️ Task With ID {} Not Found. Cannot Update.", Id);
        }
    }

    /// Removes a task from the manager by its ID.
    fn RemoveTask(&mut self, Id: u32) {
        if self.Tasks.remove(&Id).is_some() {
            println!("🗑️ Task {} Removed Successfully.", Id);
        } else {
            println!("⚠️ Task With ID {} Not Found. Cannot Remove.", Id);
        }
    }

    /// Lists all tasks currently in the manager.
    fn ListAllTasks(&self) {
        if self.Tasks.is_empty() {
            println!("\nℹ️ No Tasks Available. Add Some Tasks First!");
            return;
        }
        println!("\n--- 📋 All Tasks ---");
        // Sort tasks by ID for consistent display order
        let mut SortedTasks: Vec<&Task> = self.Tasks.values().collect();
        SortedTasks.sort_by_key(|Task| Task.Id);

        for CurrentTask in SortedTasks {
            println!("-------------------------------");
            println!(
                "🆔: {}, Description: \"{}\", Status: {:?}",
                CurrentTask.Id, CurrentTask.Description, CurrentTask.Status
            );
        }
        println!("-------------------------------\n");
    }
}

/// Helper function to get trimmed user input from the console.
fn GetUserInput(Prompt: &str) -> String {
    print!("{}", Prompt);
    io::stdout().flush().expect("Failed To Flush Stdout.");
    let mut Input = String::new();
    io::stdin()
        .read_line(&mut Input)
        .expect("Failed To Read Line From Stdin.");
    Input.trim().to_string()
}

/// Main function to run the Task Manager application.
fn main() {
    let mut MyTaskManager = TaskManager::New();

    println!("🚀 Welcome To The Rust Task Manager! 🚀");

    loop {
        println!("\n--- Main Menu ---");
        println!("1. Add New Task");
        println!("2. View Specific Task");
        println!("3. Update Task Status");
        println!("4. Remove Task");
        println!("5. List All Tasks");
        println!("6. Exit Application");

        let Choice = GetUserInput("➡️ Enter Your Choice (1-6): ");

        match Choice.as_str() {
            "1" => {
                let Description = GetUserInput("📝 Enter Task Description: ");
                if !Description.is_empty() {
                    MyTaskManager.AddTask(Description);
                } else {
                    println!("⚠️ Task Description Cannot Be Empty.");
                }
            }
            "2" => {
                let IdStr = GetUserInput("🆔 Enter Task ID To View: ");
                match IdStr.parse::<u32>() {
                    Ok(Id) => MyTaskManager.ViewTask(Id),
                    Err(_) => println!("⚠️ Invalid ID Format. Please Enter A Number."),
                }
            }
            "3" => {
                let IdStr = GetUserInput("🆔 Enter Task ID To Update: ");
                match IdStr.parse::<u32>() {
                    Ok(Id) => {
                        if MyTaskManager.Tasks.contains_key(&Id) {
                            println!("\nSelect New Status For Task {}:", Id);
                            println!("  a. Pending");
                            println!("  b. InProgress");
                            println!("  c. Completed");
                            println!("  d. Cancelled");
                            let StatusChoice = GetUserInput("➡️ Enter Status Choice (a/b/c/d): ");
                            let NewStatusResult = match StatusChoice.to_lowercase().as_str() {
                                "a" => Some(TaskStatus::Pending),
                                "b" => Some(TaskStatus::InProgress),
                                "c" => Some(TaskStatus::Completed),
                                "d" => Some(TaskStatus::Cancelled),
                                _ => {
                                    println!("⚠️ Invalid Status Choice.");
                                    None
                                }
                            };
                            if let Some(Status) = NewStatusResult {
                                MyTaskManager.UpdateTaskStatus(Id, Status);
                            }
                        } else {
                        println!("⚠️ Task With ID {} Not Found.", Id);
                        }
                    }
                    Err(_) => println!("⚠️ Invalid ID Format. Please Enter A Number."),
                }
            }
            "4" => {
                let IdStr = GetUserInput("🆔 Enter Task ID To Remove: ");
                match IdStr.parse::<u32>() {
                    Ok(Id) => MyTaskManager.RemoveTask(Id),
                    Err(_) => println!("⚠️ Invalid ID Format. Please Enter A Number."),
                }
            }
            "5" => {
                MyTaskManager.ListAllTasks();
            }
            "6" => {
                println!("\n👋 Exiting Task Manager. Thank You & Goodbye!");
                break; // Exit the loop and terminate the program
            }
            _ => println!("⚠️ Invalid Choice. Please Select A Valid Option From The Menu (1-6)."),
        }
    }
}
