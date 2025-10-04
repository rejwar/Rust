// Define the main function where the logic resides.
fn main() {
    // The value we are matching against.
    let dice_roll = 9;
    
    // The match expression must be exhaustive, covering all possible values of `dice_roll`.
    match dice_roll {
        // Specific pattern: If dice_roll is 3, execute this function.
        3 => add_fancy_hat(),
        
        // Specific pattern: If dice_roll is 7, execute this function.
        7 => remove_fancy_hat(),
        
        // Catch-all pattern: Binds any other value to the variable `other`.
        // This ensures the match is exhaustive and passes the remaining value to the function.
        other => move_player(other),
    }
}

// Dummy functions to simulate actions for clarity.
fn add_fancy_hat() {
    println!("Action: Add hat");
}
fn remove_fancy_hat() {
    println!("Action: Remove hat");
}
// This function takes the value bound by the 'other' pattern.
fn move_player(num_spaces: u8) {
    println!("Action: Move player {} spaces", num_spaces);
}