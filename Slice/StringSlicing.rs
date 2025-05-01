fn main() {

    let ActionHero: String = String::from("Arnod Schawrznegger");



    let FirstName:&str = &ActionHero[..6];

    println!("His first name is {}", FirstName);



    let LastName:&str = &ActionHero[7..];

    println!("His last name is {}",LastName);

}
