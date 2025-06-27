use std::collections::hash_map::Values;

fn main() {
    let mut values = vec![1,2,3,4,5,6];

    for val in values.iter_mut() {
        *val *=10; 
    }
    println!("Modified values :{:?}", values);
    
}
