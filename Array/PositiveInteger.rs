fn FirstMissingPositive(arr: &mut [i32]) -> i32 {
    let n = arr.len();
    for i in 0..n {
        while arr[i] > 0 && arr[i] <= n as i32 && arr[i] != arr[(arr[i] - 1) as usize] {
            arr.swap(i, (arr[i] - 1) as usize);
        }
    }

    for i in 0..n {
        if arr[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }

    (n + 1) as i32
}

fn main() {
    let mut arr = [3, 4, -1, 1];
    println!("First Missing Positive: {}", FirstMissingPositive(&mut arr)); // Output: 2
}
