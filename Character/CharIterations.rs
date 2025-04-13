fn main()
{
    println!("English Letters ");
    for Letter in 'A' ..= 'Z'
    {
        print!("{}" , Letter);
    }
    println!();


    println!("Bangla Letters: ");
    for Letter in 'অ'..= 'ই'
    {
        print!("{}", Letter);
    } 
    println!();

    let Text = "Hello , bangla";
    println!("Character is String");

    for Character in Text.chars() {
        print!("{}", Character);
    }
    println!();

}
