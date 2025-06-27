fn main() {
    let nums = vec![1,2,3,4,5,6,7];

    let sqaurd: Vec<_> = nums.iter().map(|x| x * x ).collect();

    println!("Squard : {:?}", sqaurd);
}
