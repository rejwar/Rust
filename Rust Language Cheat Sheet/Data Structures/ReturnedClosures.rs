fn main() {
    let empty = || ();
    let r = empty ();
    println!("Returned Closures {:?}", r);
}
