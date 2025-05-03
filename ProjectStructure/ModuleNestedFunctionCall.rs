mod outer {
    pub mod inner {
        pub fn Great() {
            println!("Hello from inner module");
        }
    }
}

fn main() {
    outer::inner::Great();
}
