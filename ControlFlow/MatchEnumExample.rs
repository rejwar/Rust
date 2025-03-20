enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;
    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        _ => println!("Other direction"),
    }
}
