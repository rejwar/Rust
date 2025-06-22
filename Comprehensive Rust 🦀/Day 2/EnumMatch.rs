enum Direction {
    North,
    South,
    West,
    East,
}

fn PrintDirection(dir :Direction) {
    match dir  {
        Direction::North => println!("You are heading to north. "),
        Direction::East  => println!("You are heading to East"),
        Direction::South => println!("You are heading to South"),
        Direction::West  => println!("You are heading to WEst"),

    }
}

fn main() {
    PrintDirection(Direction::East);
}
