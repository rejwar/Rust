fn FindIndex(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in arr.iter().enumerate() {
        if v == target {
            return Some(i);
        }
    }
    None
}
