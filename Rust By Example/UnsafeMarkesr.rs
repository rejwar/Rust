unsafe trait Marker {
    struct MyType;

    unsafe impl Marker for MyType{} 

    fn main() {
        println!("My Type implements the unsafe marker trait ");
    }
}
