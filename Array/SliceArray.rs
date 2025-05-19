fn SliceArraySubPart () {
    let arr = [1,2,3,4,5,6];
    let part = &arr[2..5];
    println!("Slice sub-array : {:?}", part);
}

fn main() {
    SliceArraySubPart();
}
