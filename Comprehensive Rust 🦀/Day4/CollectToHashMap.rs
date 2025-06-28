use std::collections::HashMap; 

fn main() {
    let pairs = vec![("one" , 1) , ("Two" , 2)];

    let map: HashMap<_ , _>  = pairs.into_iter().collect();
    println!( " {:?}", map);
}
