struct BigData {
    payload: [u8; 1024 * 8],
}

fn UseBoxLargeStruct() {
    let big = Box::new(BigData {
        payload: [0u8; 1024 * 8],
    });
    let moved = big;
    println!("Big data moved safely without copying 8kb");
}

fn main() {
    UseBoxLargeStruct();
}
