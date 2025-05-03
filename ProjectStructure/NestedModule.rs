mod outer {
    pub mod inner {
        pub fn Greet() {
            println!("Hello from innner Module");

        }
    }
}

fn main() {
    outer::inner::Greet();c
}
