fn main()
{
    let NewLine:char = '\n';
    let CarriageReturn:char ='\r';
    let Tab:char = '\t';
    let Backslash:char ='\\';
    let SingleQuote:char ='\'';

let UnicodeChar:char = '\u{0906}';

println!(" NewLine as char : {:?}" , NewLine);
println!("Unicode as char : {:?}", UnicodeChar);
println!("IS '\\n'  controll char? {}" , NewLine.is_control()); 
}
