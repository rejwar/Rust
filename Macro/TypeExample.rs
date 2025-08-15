macro_rules! declare_type {
    ($t:ty) => {
        fn print_type(val: $t) {
            println!("Got a value!");
        }
    };
}

declare_type!(u32);

fn main() {
    print_type(42); // Output: Got a value!
}
