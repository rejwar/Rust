fn Print_slice(slice: &[i32]) {
    println!("{:?}", slice);
}

fn main() {
    let arr = [1,2,3,4,5];
    Print_slice(&arr);
}
