fn NumericCasting() {
    let a: i32 = 10;
    let b: f64 = a as f64;
    println!("b = {}", b);


    let x: u8 = 255;
    let y: i16 = x as i16;

    println!("y = {}", y);
}

fn main() {
    NumericCasting();
}