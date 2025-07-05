trait Limits {
    const  Max: i32;
}

struct Speed;

impl Limits for Speed {
    const  Max: i32 = 120;
}

fn main() {
    println!("Max speed {}", Speed::Max);
}
