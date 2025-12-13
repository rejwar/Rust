fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    for num in nums.iter_mut() {
        *num *= 2;
    }

    println!("Modified : {:?}", nums);
}
