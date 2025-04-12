fn IsSorted(Numbers: &[i32]) -> bool {
    for i in 1..Numbers.len() {
        if Numbers[i - 1] > Numbers[i] {
            return false; // Not sorted
        }
    }
    true // Sorted
}

fn main() {
    let SortedData: Vec<i32> = vec![1, 2, 3, 4, 5];
    let UnsortedData: Vec<i32> = vec![5, 3, 1, 4, 2];

    println!("Is Sorted Data Sorted? {}", IsSorted(&SortedData));
    println!("Is Unsorted Data Sorted? {}", IsSorted(&UnsortedData));
}
