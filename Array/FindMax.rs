fn FindMax(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

fn main() {
    let arr = [10, 20, 5, 30, 25];
    println!("Maximum element: {}", FindMax(&arr)); // Output: 30
}
