#[derive(Debug)]

struct Color(u8 , u8 , u8);

fn main() {
    let red = Color (255,0,0);
    println!("{:?} -> R = {} , {} , {}" ,red , red.0 , red.1, red.2);
}