fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();

    println!("First is {}", nums[0]);

    for n in nums.iter() {
        println!("{}", n);
    }
}
