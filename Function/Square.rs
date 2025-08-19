fn main() {
    let result = square(4);
    println!("Square = {}", result);
}

fn square(x: i32 ) -> i32 {
    x *x
}