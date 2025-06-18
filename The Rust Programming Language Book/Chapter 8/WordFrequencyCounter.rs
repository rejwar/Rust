use std::collections::HashMap;

fn WordFrequencyCounter() {
    let text = "Hello world hello rust";
    let mut freq = HashMap::new();

    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) +=1;
    }
    for (word , count) in freq {
        println!( "{} :{}" , word, count);
    }
}

fn main() {
    WordFrequencyCounter();
}
