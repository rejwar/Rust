fn FindTripletSum(arr: &mut [i32], target: i32) -> Option<(i32, i32, i32)> {
    arr.sort_unstable();
    let n = arr.len();

    for i in 0..n - 2 {
        let (mut left, mut right) = (i + 1, n - 1);
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == target {
                return Some((arr[i], arr[left], arr[right]));
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    None
}

fn main() {
    let mut arr = [12, 3, 4, 1, 6, 9];
    let target = 24;
    match FindTripletSum(&mut arr, target) {
        Some(triplet) => println!("Triplet found: {:?}", triplet),
        None => println!("No triplet found"),
    }
}
