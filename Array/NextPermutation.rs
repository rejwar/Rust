fn NextPermutation(arr: &mut [i32]) {
    let n = arr.len();
    let mut i = n - 2;

    while i > 0 && arr[i] >= arr[i + 1] {
        i -= 1;
    }

    if arr[i] < arr[i + 1] {
        let mut j = n - 1;
        while arr[j] <= arr[i] {
            j -= 1;
        }
        arr.swap(i, j);
    }

    arr[i + 1..].reverse();
}

fn main() {
    let mut arr = [1, 2, 3];
    NextPermutation(&mut arr);
    println!("Next Permutation: {:?}", arr);
}
