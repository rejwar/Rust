// Kaizen Commit 4: ExpressionSummer macro with controlled recursion

macro_rules! ExpressionSummer {                // Define macro
    ($X:expr) => {$X};                         // Base case: single expression
    ($X:expr, $($Rest:expr),+) => {            // Match first expr + rest
        $X + ExpressionSummer!($($Rest),+)     // Add recursively
    };
}

fn main() {                                    // PascalCase main
    let Total = ExpressionSummer!(1, 2, 3, 4, 5); // Sum 1+2+3+4+5
    println!("{}", Total);                     // Output: 15
}
