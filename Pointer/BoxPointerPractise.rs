fn main() {
    let mut HeapValue = Box::new(50);

    *HeapValue += 25;

    println!("Updated HeapValue is {}", HeapValue);
}