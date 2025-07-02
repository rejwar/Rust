fn PrintSlice(data: &[i32])
{
    println!("Slice is = {:?}", data);
}

fn main() {
    let arr = [10,20,30,40,50];

    PrintSlice(&arr);
    PrintSlice(&arr[1..4]);
}
