fn CheckCharProperties(c: char){
    println!("Is Alphabetic ?{}", c.is_alphabetic());
    println!(" Is Uppercase ?{}",c.is_uppercase());
    println!("IS Numeric?{}", c.is_numeric());
    println!(" Is WhiteSpace ?{}", c.is_whitespace());
}

fn main() {
    CheckCharProperties('E');
}
