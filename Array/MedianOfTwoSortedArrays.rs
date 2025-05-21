fn MedianOfTwoSortedArrays(arr1: &[i32], arr2: &[i32]) -> f64 {
    let mut merged = arr1.to_vec();
    merged.extend_from_slice(arr2);
    merged.sort();

    let mid = merged.len() / 2;
    if merged.len() % 2 == 0 {
        (merged[mid - 1] as f64 + merged[mid] as f64) / 2.0
    } else {
        merged[mid] as f64
    }
}

fn main() {
    let arr1 = [1, 3, 8];
    let arr2 = [7, 9, 10, 11];
    println!("Median: {}", MedianOfTwoSortedArrays(&arr1, &arr2));
}
