use rayon::prelude::*;

fn main() {
    let nums: Vec<u32> = (1 ..= 1_000_000).collect();


    let sum: u32 = nums.par_iter().sum();
    println!("Sum = {}", sum);

}