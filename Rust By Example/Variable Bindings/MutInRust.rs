fn main() {
    let mut x =10;
    let y = &mut x;

    *y +=5;

    println!("THe value  is {}", x);
}
