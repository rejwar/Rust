use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();
    data.insert("Id", 95); 
    data.insert("age", 23);

    for (key, value) in &data {
        println!( "{} => {}" , key , value);
    }
}
