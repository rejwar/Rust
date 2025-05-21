fn FindSubarraySum(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut sum = 0;
    let mut start = 0;

    for (end, &num) in arr.iter().enumerate() {
        sum += num;

        while sum > target {
            sum -= arr[start];
            start += 1;
        }

        if sum == target {
            return Some((start, end));
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 7, 5];
    let target = 12;
    match FindSubarraySum(&arr, target) {
        Some((start, end)) => println!("Subarray found from index {} to {}", start, end),
        None => println!("No subarray found"),
    }
}
