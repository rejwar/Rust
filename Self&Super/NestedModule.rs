mod outer {
    use crate::outer;

    pub mod inner {
        pub fn hi() {
            println!("Hi from inner");
        }
        pub fn greet() {
            self::hi();
        }
    }
}

fn main() {
    outer::inner::greet();
}
