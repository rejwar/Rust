fn main(){
    let greeting = String:: from ("Hello World");

    println!("{}", greeting);

    let char1 = greeting.chars().nth(1000);

    match char1{


    print!(" char1: {}",char1.unwrap());
}
}

