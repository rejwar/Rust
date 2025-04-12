fn main()
{
    let MyChar:char = 'a';
    println!("Is '{}' alphabatic {}", MyChar, MyChar.is_alphabetic());
    println!("Uppercase of '{}' is '{}'", MyChar, MyChar.to_uppercase());


    let BanglaChar:char = 'ক';
    println!("Is '{}' alphabetic? {}", BanglaChar, BanglaChar.is_alphabetic());
    
    let DigitCharL:char = '7';
    println!("IS '{}' a digit (base 10)? {}", DigitCharL , DigitCharL.is_digit(10));

    let SpaceChar:char = ' ';
    println!("IS '{}' white space {}", SpaceChar, SpaceChar.is_whitespace());
}
