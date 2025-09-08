fn main() {
    let mut nums = [1,2,3,4,5,6,7];
    let (left , right) = nums.split_at_mut(3);

    left[0] = 10;
    right[3] = 20;

    println!("{:?}" , nums);
}