fn MajorityElement(arr: &[i32]) -> i32 {
    let mut candidate = arr[0];
    let mut count = 1;

    for &num in arr.iter().skip(1) {
        if count == 0 {
            candidate = num;
        }
        count += if num == candidate { 1 } else { -1 };
    }

    candidate
}

fn main() {
    let arr = [3, 3, 4, 2, 3, 3, 3, 2];
    println!("Majority Element: {}", MajorityElement(&arr)); // Output: 3
}
