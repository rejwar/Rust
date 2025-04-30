fn GetCoordinates() -> (i32 , i32) {
    (10,20)
}

fn main() {
    let (x ,y): (i32 , i32) = GetCoordinates();
    println!("X : {} , Y : {}", x,y);
}
