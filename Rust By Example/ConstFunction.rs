const fn square(x: i32 ) -> i32 {
    x * x
}

const Area: i32 = square(5);

fn main() {
    println!("Area {}", Area);
}
