fn BinarySearch(arr: &[i32], target: i32) -> bool {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return true;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

fn main() {
    let SortedArray = [1, 3, 5, 7, 9, 11];
    println!("Found: {}", BinarySearch(&SortedArray, 7)); // ✅ O(log n) Search
}
