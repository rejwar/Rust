use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("steak"), 25.99);
    menu.insert(String::from("salad"), 9.99);
    menu.insert(String::from("soup"), 5.99);

    println!("Menu: {:?}", menu);
}
