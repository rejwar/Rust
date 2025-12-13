// FileName: BoxAlloc.rs
fn main() {
    let b = Box::new(100); // 100 is on Heap
    println!("Heap val: {}", b);
}
