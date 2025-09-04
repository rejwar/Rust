use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug , Eq , PartialEq)]

struct Product{
    name: String,
    price: i32,
}

fn main() {
    let mut products: BTreeMap<Product , i32>  = BTreeMap::new();


    products.insert(Product {name: "Laptop".to_string(), price: 1000} , 10);
    products.insert(Product { name: "Phone".to_string(),price: 500 }, 20);
    products.insert(Product { name: "Tablet ".to_string(),price: 500 }, 15);

    for (product , stock) in &products {
        println!("{} ({}$) -> {} in stock  ", product.name , product.price , stock);
    }
}