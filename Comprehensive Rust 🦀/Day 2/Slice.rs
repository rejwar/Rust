fn main() {
    let a: [i32; 6 ] = [10,20,30,40,50,60];
    println!("a : {a:?}");

    let s: &[i32] = &a[2..4];

    println!("S: {s:?}");
}
