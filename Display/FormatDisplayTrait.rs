#![allow(unused)]
fn main() {
    use std::fmt;

    struct Structure(i32);

    impl fmt::Display for Structure  {
        // This trait requires the use of the `fmt` method
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // The `write!` macro is used to write formatted data to a buffer
            write!(f, "{}", self.0)
        }
    }
}
