enum Direction {
    North,
    South,
    East,
    West,
}

fn ShowDirection (dir: Direction) {
    match dir {
        Direction::North => println!("Going North"),
        Direction::South =>println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}

fn main() 
{
    ShowDirection(Direction::East);
}
