fn main() {
    // 1. Unsigned integer
    let PositiveNumber: u32 = 12345;
    println!("PositiveNumber: {}", PositiveNumber);

    // 2. Signed integer
    let NegativeNumber: i32 = -54321;
    println!("NegativeNumber: {}", NegativeNumber);

    // 3. Large floating-point number
    let LargeFloat: f64 = 1.23456789e+30;
    println!("LargeFloat: {}", LargeFloat);

    // 4. Small floating-point number
    let SmallFloat: f32 = 1.23e-5;
    println!("SmallFloat: {}", SmallFloat);

    // 5. Boolean with condition
    let IsEven: bool = PositiveNumber % 2 == 0;
    println!("IsEven: {}", IsEven);

    // 6. Character
    let SpecialChar: char = '@';
    println!("SpecialChar: {}", SpecialChar);

    // 7. String slice
    let StaticMessage: &str = "Hello from a slice!";
    println!("StaticMessage: {}", StaticMessage);

    // 8. Vector
    let NumberVector: Vec<i32> = vec![10, 20, 30];
    println!("NumberVector: {:?}", NumberVector);

    // 9. HashMap (dictionary)
    use std::collections::HashMap;
    let mut NameToAge: HashMap<&str, u32> = HashMap::new();
    NameToAge.insert("Alice", 30);
    NameToAge.insert("Bob", 25);
    println!("NameToAge: {:?}", NameToAge);

    // 10. Option type
    let MaybeNumber: Option<u32> = Some(42);
    println!("MaybeNumber: {:?}", MaybeNumber);

    // 11. Result type
    let Calculation: Result<u32, &str> = Ok(10 + 20);
    println!("Calculation: {:?}", Calculation);

    // 12. Empty tuple (unit type)
    let UnitValue: () = ();
    println!("UnitValue: {:?}", UnitValue);

    // 13. Reference variable
    let Original = 42;
    let ReferenceToOriginal: &i32 = &Original;
    println!("ReferenceToOriginal: {}", ReferenceToOriginal);

    // 14. Mutable reference
    let mut MutableData = 100;
    {
        let MutableReference: &mut i32 = &mut MutableData;
        *MutableReference += 50;
    }
    println!("MutableData: {}", MutableData);

    // 15. Shadowing with type change
    let ShadowedVariable = 5;
    let ShadowedVariable = "Now a String!";
    println!("ShadowedVariable: {}", ShadowedVariable);

    // 16. Array with a fixed size
    let FixedSizeArray: [i32; 3] = [1, 2, 3];
    println!("FixedSizeArray: {:?}", FixedSizeArray);

    // 17. Slice from an array
    let ArraySlice: &[i32] = &FixedSizeArray[0..2];
    println!("ArraySlice: {:?}", ArraySlice);

    // 18. Struct
    struct Person {
        Name: String,
