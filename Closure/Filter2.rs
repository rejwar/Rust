fn main() {
    let words = vec!["Banana", "apple" , "Orange" , "Sugar"];

    let filtered_words: Vec<_> = words.into_iter().filter(|words| words.contains('a')).collect();

    println!("Words containing 'a' : {:.?}", filtered_words);
}