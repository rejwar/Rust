fn reverse_array(arr: &mut [i32]) {
    arr.reverse();
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    println!("{:?}", arr); // Output: [5, 4, 3, 2, 1]
}
