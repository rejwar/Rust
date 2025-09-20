use core::num;

use rayon::{prelude::*, vec};

fn main() {
    let mut nums = vec![10, 4, 5 , 6,7,8,9,11,6];

    nums.par_sort();
    println!("{:?}", nums);
}