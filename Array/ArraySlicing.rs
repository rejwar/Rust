use core::slice;

fn main() {
    let Numbers: [i32; 5] = [10,20,30,40,50];

    let Slice = &Numbers[1..4];
    println!("Slice {:?}", Slice);

    let FullSlice = &Numbers[..];
    println!("Full slice: {:?}", FullSlice);

    let StartSlice = &Numbers[..3];
    println!("Start slice: {:?}", StartSlice);

    let EndSlice = &Numbers[2..];
    println!("End slice: {:?}", EndSlice);

    PrintSlice(Slice);
}

fn PrintSlice(Slice: &[i32]){
    println!("slice in function: {:?}",Slice);
    println!("Slice length: {}", Slice.len());
}
