fn main()
{
    let mut CharVector: Vec<char> = Vec::new();


    CharVector.push('H');
    CharVector.push('E');
    CharVector.push('L');
    CharVector.push('L');
    CharVector.push('O');

    println!("Character vector : {:?}", CharVector);

    let StringFromChar:String = CharVector.iter().collect();
    println!("String from chars: {}", StringFromChar);


    CharVector.reverse();
    let ReveredString:String = CharVector.iter().collect();
    println!("Reversed: {}", ReveredString);
}
