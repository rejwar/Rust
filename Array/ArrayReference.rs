fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let total = sum(&arr);

    println!("Sum: {}", total);
}
