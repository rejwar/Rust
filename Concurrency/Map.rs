use rayon::prelude::*;

fn main() {
    let nums: Vec<u32> = (1..= 10).collect();

    let squares: Vec<u32> = nums. par_iter().map(|x| x * x).collect();

    println!(" {:?}", squares);
}