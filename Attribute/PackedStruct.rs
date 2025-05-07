#[repr(C, packed)]
struct PackedData {
    Field1: u8,
    Field2: u32,
}

fn main() {
    println!("Size of PackedData: {}", std::mem::size_of::<PackedData>());
}
