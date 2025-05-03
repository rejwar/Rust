use std::collections::HashMap;

fn main() {
    // প্রথম HashMap: মেনু আইটেম এবং তাদের দাম
    let mut menu = HashMap::new();
    menu.insert(String::from("steak"), 25.99);
    menu.insert(String::from("salad"), 9.99);
    menu.insert(String::from("soup"), 5.99);
    println!("Menu: {:?}", menu);

    // দ্বিতীয় HashMap: দেশ এবং রাজধানী
    let mut country_capital = HashMap::new();
    country_capital.insert("France", "Paris");
    country_capital.insert("Japan", "Tokyo");
    country_capital.insert("Brazil", "Brasilia");
    println!("Country capitals: {:?}", country_capital);
}
