// Question: How do we define and use a basic enum in Rust?

enum Direction {
    North,
    South,
    East,
    West,
}

fn MatchDirection() {
    let dir = Direction::East;

    match dir {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}
fn main() {
    MatchDirection();
}
