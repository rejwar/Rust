fn FindKthSmallestElement(arr: &mut [i32], k: usize) -> Option<i32> {
    arr.sort_unstable();
    arr.get(k - 1).copied()
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    match FindKthSmallestElement(&mut arr, k) {
        Some(element) => println!("K-th smallest element: {}", element),
        None => println!("Invalid K value"),
    }
}
