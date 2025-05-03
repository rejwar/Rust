use std::collections::{btree_map::Values, HashMap};

fn main() {
    let mut CoffeePairing: HashMap<&str , &str> = HashMap::new();
    let Drink = String::from("Latte");
    let Milk= String::from("Oat Milk");
    CoffeePairing.insert(&Drink, &Milk);
    CoffeePairing.insert("Flat White", "Almond Milk");
    CoffeePairing.insert("Latte",  "Almond Milk");
    println!("{:?}" , CoffeePairing);
    CoffeePairing.insert("Latte",  "Pistachio Milk");
    println!("{:?}" , CoffeePairing);
}
