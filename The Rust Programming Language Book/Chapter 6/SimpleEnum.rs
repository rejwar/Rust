enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn Move(Direction: Direction) {
    match Direction {
        Direction::Up => println!("Moving Up!"),
        Direction::Down => println!("Moving Down!"),
        Direction::Left => println!("Moving Left!"),
        Direction::Right => println!("Moving Right!"),
    }
}

fn main() {
    let Dir = Direction::Up;
    Move(Dir); // ফলাফল: Moving Up!
}
