fn main()
{
    let Message:String = String::from ("Rust");


    let CharIterator  = Message.chars();

    for Character in CharIterator {
        println!("Character : {}" , Character);
    }

    let NthChar = Message.chars().nth(1);
    match NthChar {
        Some(Character) => println!("Second Character : {}", Character),
        None => println!("Index Out of boxes"),
    }

}
