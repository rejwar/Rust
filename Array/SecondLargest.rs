fn SecondLargest(arr: &[i32]) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_unstable();
    sorted_arr.iter().rev().nth(1).copied()
}

fn main() {
    let arr = [10, 20, 5, 30, 25];
    match SecondLargest(&arr) {
        Some(val) => println!("Second largest element: {}", val),
        None => println!("Array is too small"),
    }
}
