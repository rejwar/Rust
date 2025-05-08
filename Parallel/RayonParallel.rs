use rayon::prelude::*;

fn main() {
    let Numbers = vec![1, 2, 3, 4, 5];

    let SquaredNumbers: Vec<i32> = Numbers.par_iter().map(|x| x * x).collect();

    println!("Squared Numbers (Parallel Execution): {:?}", SquaredNumbers);
}
