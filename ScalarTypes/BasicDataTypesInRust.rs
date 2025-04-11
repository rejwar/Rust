// Main Function In PascalCase
fn Main() {
    // Integer Types In PascalCase
    let PositiveNumber: i32 = 42;
    let NegativeNumber: i64 = -5000;
    let SmallNumber: u8 = 255;
    let BigNumber: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    // Floating Point Types In PascalCase
    let FloatValue: f32 = 3.14;
    let DoubleValue: f64 = 2.71828;

    // Boolean Type In PascalCase
    let IsRustAwesome: bool = true;

    // Character Type In PascalCase
    let UnicodeChar: char = '😊';

    // Tuple Type In PascalCase
    let TupleData: (i32, f64, char) = (42, 3.14, 'R');

    // Array Type In PascalCase
    let NumberArray: [i32; 5] = [1, 2, 3, 4, 5];

    // Printing Values
    PrintValues(
        PositiveNumber,
        NegativeNumber,
        SmallNumber,
        BigNumber,
        FloatValue,
        DoubleValue,
        IsRustAwesome,
        UnicodeChar,
        TupleData,
        NumberArray,
    );
}

// Helper Function In PascalCase
fn PrintValues(
    PosNum: i32,
    NegNum: i64,
    SmNum: u8,
    BgNum: u128,
    FlVal: f32,
    DbVal: f64,
    IsRustGood: bool,
    UniChar: char,
    Tpl: (i32, f64, char),
    Arr: [i32; 5],
) {
    println!("Positive Integer: {}", PosNum);
    println!("Negative Integer: {}", NegNum);
    println!("Small Unsigned Integer: {}", SmNum);
    println!("Big Unsigned Integer: {}", BgNum);
    println!("Float Value: {}", FlVal);
    println!("Double Value: {}", DbVal);
    println!("Is Rust Awesome: {}", IsRustGood);
    println!("Unicode Character: {}", UniChar);
    println!("Tuple: {:?}", Tpl);
    println!("Array: {:?}", Arr);
}
