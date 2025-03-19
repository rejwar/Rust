#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());

    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        _ => println!("Other direction"),
    }
}
