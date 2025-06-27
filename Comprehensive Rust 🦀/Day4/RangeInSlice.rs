use core::slice;

fn main() {
    let arr = [10,20,30,40];

    let slice = &arr[1..4];

    println!(" Slice {:?} ", slice);
}
