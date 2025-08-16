// Kaizen Commit 2: RepeatPrinter macro with recursive TT munching

macro_rules! RepeatPrinter {                    // Define macro
    () => {};                                   // Base case: nothing left, stop
    ($Word:expr; $Token:tt $($Rest:tt)*) => {   // Match head token, recurse on tail
        println!("{}", $Word);                  // Print word once
        RepeatPrinter!($Word; $($Rest)*);       // Continue recursion
    };
}

fn main() {                                     // Always lowercase main
    RepeatPrinter!("Hello"; 1 2 3 4);           // Pass 4 tokens to control recursion
    // Output: Hello (x4)
}
