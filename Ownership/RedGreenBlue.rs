enum Direction {
    Red,
    Yellow,
    Green,
}

fn PrintCode(c: Direction) {
    match c {
        Direction::Green => println!("Go"),
        Direction::Red => println!("Don't Go"),
        Direction::Yellow => println!("NO moving"),
    }
}
fn main() {}
