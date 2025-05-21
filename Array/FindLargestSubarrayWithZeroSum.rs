use std::collections::HashMap;

fn FindLargestSubarrayWithZeroSum(arr: &[i32]) -> Option<(usize, usize)> {
    let mut sum_map = HashMap::new();
    let mut sum = 0;
    let mut max_len = 0;
    let mut start_end = None;

    for (i, &num) in arr.iter().enumerate() {
        sum += num;

        if sum == 0 {
            max_len = i + 1;
            start_end = Some((0, i));
        }

        if let Some(&prev_index) = sum_map.get(&sum) {
            if i - prev_index > max_len {
                max_len = i - prev_index;
                start_end = Some((prev_index + 1, i));
            }
        } else {
            sum_map.insert(sum, i);
        }
    }

    start_end
}

fn main() {
    let arr = [4, 2, -3, 1, 6, -3, 2, -2, 4, -6];
    match FindLargestSubarrayWithZeroSum(&arr) {
        Some((start, end)) => println!("Largest subarray with zero sum: {} to {}", start, end),
        None => println!("No subarray found"),
    }
}
