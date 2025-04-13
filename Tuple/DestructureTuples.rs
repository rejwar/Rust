fn DestructureTuple() {
    let person = ("Alice" , 30);

    let (name, age) = person;
    
    println!("Name : {} , Age: {}", name,age );
}

fn main() 
{
    DestructureTuple();
}
