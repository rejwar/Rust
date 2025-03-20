fn main() {
    // 1. Immutable variable
    let MyVariable = 10;
    println!("MyVariable: {}", MyVariable);

    // 2. Mutable variable
    let mut MutableVariable = 20;
    MutableVariable += 5;
    println!("MutableVariable: {}", MutableVariable);

    // 3. Floating-point variable
    let DecimalNumber = 3.14;
    println!("DecimalNumber: {}", DecimalNumber);

    // 4. Boolean variable
    let IsActive = true;
    println!("IsActive: {}", IsActive);

    // 5. Character variable
    let Letter = 'R';
    println!("Letter: {}", Letter);

    // 6. String variable
    let Greeting = String::from("Hello, Rust!");
    println!("Greeting: {}", Greeting);

    // 7. Array variable
    let NumbersArray = [1, 2, 3, 4, 5];
    println!("NumbersArray: {:?}", NumbersArray);

    // 8. Tuple variable
    let MyTuple = (10, 3.14, "Tuple");
    println!("MyTuple: {:?}", MyTuple);

    // 9. Constant variable
    const MAX_LIMIT: u32 = 100;
    println!("MAX_LIMIT: {}", MAX_LIMIT);

    // 10. Shadowing variable
    let ShadowedVariable = 50;
    let ShadowedVariable = ShadowedVariable + 10; // New scope with the same name
    println!("ShadowedVariable: {}", ShadowedVariable);
}
