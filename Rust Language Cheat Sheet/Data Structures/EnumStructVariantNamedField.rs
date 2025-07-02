

enum Action {
    Move { x: i32, y: i32 },
    Stop,
}

fn main() {
    let action = Action::Move { x: 5, y: 10 };

    match action {
        Action::Move { x: x_pos, y: y_pos } => {
            println!("Moving to ({}, {})", x_pos, y_pos);
        }
        Action::Stop => {
            println!("Stopping.");
        }
    }
}
