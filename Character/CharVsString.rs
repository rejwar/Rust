fn main()
{
    let SingleChar:char = 'a';
    let SingleString:&str = "a";

    let StringObject: String = String::from("A");

    println!("Char size {} bytes ",std::mem::size_of::<char>());
    println!("String Literal is UTF-8 encode ");

}
