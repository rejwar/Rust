fn main() {
    let (mut x,y)=(1, 2); // This is a comment
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("x: {}, y: {}", x, y);
}
