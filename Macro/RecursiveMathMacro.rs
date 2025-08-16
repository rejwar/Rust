// RecursiveMathMacro.rs

// Kaizen Step 1: Define macro with recursion
macro_rules! RecursiveSum {                                     // Define macro for summation
    ($x:expr) => { $x };                                        // Base case: only one element
    ($x:expr, $($rest:expr),+) => { $x + RecursiveSum!($($rest),+) }; // Recursive step: add first + rest
}

macro_rules! RecursiveProduct {                                 // Define macro for product
    ($x:expr) => { $x };                                        // Base case: only one element
    ($x:expr, $($rest:expr),+) => { $x * RecursiveProduct!($($rest),+) }; // Recursive step: multiply first * rest
}

fn Main() {                                                     // Entry point
    let result_sum = RecursiveSum!(1, 2, 3, 4, 5);              // Should expand to ((((1+2)+3)+4)+5)
    let result_product = RecursiveProduct!(1, 2, 3, 4, 5);      // Should expand to ((((1*2)*3)*4)*5)

    println!("Sum Result = {}", result_sum);                    // Defensive check output
    println!("Product Result = {}", result_product);            // Defensive check output
}
