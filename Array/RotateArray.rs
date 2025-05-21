fn RotateArray(arr: &mut [i32], k: usize) {
    let len = arr.len();
    arr.rotate_right(k % len);
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    RotateArray(&mut arr, 2);
    println!("{:?}", arr); // Output: [4, 5, 1, 2, 3]
}
