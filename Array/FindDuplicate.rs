use std::collections::HashSet;

fn FindDuplicate(arr: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for &num in arr {
        if !seen.insert(num) {
            duplicates.push(num);
        }
    }

    duplicates
}

fn main() {
    let arr = [1, 2, 3, 4, 2, 5, 3];
    println!("Duplicate elements: {:?}", FindDuplicate(&arr)); // Output: [2, 3]
}
