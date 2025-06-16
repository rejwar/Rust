// Question: How do we define a basic enum and use pattern matching to act based on its variants?

enum Direction {
    North,
    South,
    East,
    West,
}

fn MatchDirection() {
    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going up!"),
        Direction::South => println!("Going down!"),
        Direction::East => println!("Going right!"),
        Direction::West => println!("Going left!"),
    }
}
