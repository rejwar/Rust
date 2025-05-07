#[repr(transparent)]
struct Wrapper(u32);

fn main() {
    let WrappedValue = Wrapper(42);
    println!("Wrapped Value: {}", WrappedValue.0);
}
