fn main()

{
    let TestChar:char = 'A';

    println!("IS alphabetic: {}", TestChar.is_alphabetic());
    println!("Is Numeric:{}",TestChar.is_numeric());
    println!("IS alphanumric {}", TestChar.is_alphanumeric());
    println!("Is whiteSpace {}", TestChar.is_whitespace());
    println!("Is control {}" , TestChar.is_control());

    println!("Lowercase of {}, is {}", TestChar, TestChar.to_lowercase());
    println!("UpperCase of {}, is {} ", TestChar, TestChar.to_uppercase());

    if TestChar.is_ascii()
    {
        println!("ASCII VAlue : {}" , TestChar as u8);
    }

    let NumberAsChar:char = 65 as char ;
    println!("Char from number 65: {}", NumberAsChar);
}
