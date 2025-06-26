use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();

    let words = vec!["a", "b" , "c" , "d"];

    for word in words {
        *counts.entry(word).or_insert(0) +=1 ;

    }

    println!( " {:?}", counts);
}
