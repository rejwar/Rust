enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("You are heading North!"),
        Direction::South => println!("You are heading South!"),
        Direction::East => println!("You are heading East!"),
        Direction::West => println!("You are heading West!"),
    }
}
