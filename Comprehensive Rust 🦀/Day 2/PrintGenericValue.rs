fn PrintValue<T : std::fmt::Debug>(value: T) {
    println!("{:?}", value);
} 

fn main() {
    PrintValue(42);
    PrintValue("Hello Wprld");
    PrintValue(vec![1,2,3,4,5,6]);
}
