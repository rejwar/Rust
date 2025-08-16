// Kaizen Commit 2: ExpressionSummer macro with recursive addition

macro_rules! ExpressionSummer {                       // Define macro
    ($X:expr) => {$X};                                // Base case: single expression
    ($X:expr, $($Rest:expr),+) => {                   // Recursive case: head expr + tail
        $X + ExpressionSummer!($($Rest),+)            // Sum recursively
    };
}

fn main() {                                           // Always lowercase main
    let Total = ExpressionSummer!(10, 20, 30);        // Sum 10+20+30
    println!("{}", Total);                            // Output: 60
}
