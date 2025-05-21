fn EquilibriumIndex(arr: &[i32]) -> Option<usize> {
    let total_sum: i32 = arr.iter().sum();
    let mut left_sum = 0;

    for (i, &num) in arr.iter().enumerate() {
        if left_sum == total_sum - left_sum - num {
            return Some(i);
        }
        left_sum += num;
    }

    None
}

fn main() {
    let arr = [-7, 1, 5, 2, -4, 3, 0];
    match EquilibriumIndex(&arr) {
        Some(index) => println!("Equilibrium Index: {}", index), // Output: 3
        None => println!("No Equilibrium Index Found"),
    }
}
