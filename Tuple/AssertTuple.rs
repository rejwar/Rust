fn main() {
    let tuple: (i32, f64, char) = (42, 3.14, 'a');
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let (a,b,c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
}
