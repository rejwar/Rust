fn main() {
    let outer = 42;
     {
        let inner =10;
        println!("Inner {}, Outer {}", inner, outer );
     }
     println!("Outer {} " ,outer);
}
