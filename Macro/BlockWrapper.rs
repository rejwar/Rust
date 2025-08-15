// This macro takes a block of code and executes it with clear headers
macro_rules! BlockWrapper {
    ($CodeBlock:block) => {
        println!("--- Start of Code Block ---");
        $CodeBlock
        println!("--- End of Code Block ---\n");
    };
}

fn RunBlockProfessional() {
    // Example 1: simple block
    BlockWrapper!({
        let tempValue: i32 = 10;
        println!("TempValue inside block: {}", tempValue);
    });

    // Example 2: multiple statements in block
    BlockWrapper!({
        let mut counter: i32 = 0;
        counter += 7;
        println!("Counter after increment: {}", counter);
        println!("All operations inside professional block completed!");
    });
}

fn main() {
    RunBlockProfessional();
}
