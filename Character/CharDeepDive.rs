// ফাইল: CharDeepDive.rs
fn Main() {
    // ইউনিকোড স্ক্যালার ভ্যালু
    let HeartEmoji: char = '❤';
    println!("Heart emoji Unicode: U+{:X}", HeartEmoji as u32);
    
    // বাইট কনভার্শন (ASCII সীমার মধ্যে)
    let AsciiChar: char = 'A';
    let ByteValue: u8 = AsciiChar as u8;
    println!("'A' as byte: {}", ByteValue);
    
    // সার্জিং (সাবধানতা অবলম্বন করুন)
    let NonAsciiChar: char = 'ঞ';
    // println!("Non-ASCII as byte: {}", NonAsciiChar as u8); // পেনিক করবে!
    
    // ক্যাস্টিং
    let NumberValue: u32 = 66;
    let CharFromNumber: char = char::from_u32(NumberValue).unwrap();
    println!("Char from 66: {}", CharFromNumber);
    
    // Option<char> হ্যান্ডলিং
    let ValidChar = char::from_u32(65);
    let InvalidChar = char::from_u32(0x110000); // ইউনিকোড রেঞ্জের বাইরে
    
    match ValidChar {
        Some(Character) => println!("Valid char: {}", Character),
        None => println!("Invalid Unicode value"),
    }
    
    match InvalidChar {
        Some(Character) => println!("Valid char: {}", Character),
        None => println!("Invalid Unicode value"),
    }
}
