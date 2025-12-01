enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Left;

    match dir {
        Direction::Up => println!("[translate:up]"),
        Direction::Down => println!("[translate:down]"),
        Direction::Left => println!("[translate:left]"),
        Direction::Right => println!("[translate:right]"),
    }
}
