enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Down;

    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}
