use std::collections::HashMap;

fn main() {
    let mut CoffeePairing: HashMap<&str , &str> = HashMap::new();
    let Drink = String::from("Latte");
    let Milk= String::from("Oat Milk");
    CoffeePairing.insert(&Drink, &Milk);
    CoffeePairing.insert("Flat White", "Almond Milk");
}
