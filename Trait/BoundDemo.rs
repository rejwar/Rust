trait Debuggable {
    fn display_info(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Debuggable for Point {
    fn display_info(&self) -> String {
        format!("Point at {} {} ", self.x, self.y)
    }
}

struct Size {
    width: u32,
    height: u32,
}

impl Debuggable for Size {
    fn display_info(&self) -> String {
        format!(" Size {} {} ", self.width, self.height)
    }
}

fn debug_print<T: Debuggable>(item: &T) {
    let info = item.display_info();
    println!("Debug {}", info);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let s = Size {
        width: 100,
        height: 50,
    };

    println!("------- Debugging--------");
    debug_print(&s);
}
