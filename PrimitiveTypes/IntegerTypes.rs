fn Main()

{
    let EnglishChar:char = 'A';
    let BengaliChar:char = 'এ';
    let EmojiChart:char = '🤣';

    let UnicodeChar:char = '\u{0986}';

    println!("English Character: {}", EnglishChar);
    println!("Bengali Character: {}",BengaliChar);
    println!("Emoji Character:{} ", EmojiChart);
    println!("Unicode Character: {}", UnicodeChar);

    println!("'A' as u32: {}" , EnglishChar);

}

fn main()

{
    Main();
}
