fn takes_u32(x: u32) {
    println!("Received u32: {}", x);
}

fn takes_u8 (x: u8) {
    println!("Received u8: {}", x);
}

fn main() {
    let x = 10;
    let y: i32 =20;

    takes_u32(x as u32);
    takes_u8(y as u8);
}
