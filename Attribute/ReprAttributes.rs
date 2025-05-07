#[repr(C)]
struct Data {
    Value1: u8,
    Value2: u32,
}

fn main() {
    println!("Size of Data: {}", std::mem::size_of::<Data>());
}
