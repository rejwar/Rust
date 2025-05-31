fn main() {
    let (mut x, mut y): (i32, i32);

    let (x,..) = (3,4);
    [..,y] = [1,2];
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}
