// file: group_anagrams.rs

use std::collections::HashMap;

fn GroupAnagrams(words: Vec<&str>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let mut chars: Vec<char> = word.chars().collect();

        chars.sort();

        let key: String = chars.into_iter().collect();

        map.entry(key).or_insert(Vec::new()).push(word.to_string());
    }

    map.into_values().collect()
}

fn main() {
    let words = vec!["eat", "tea", "tan", "ate", "nat", "bat"];

    let result = GroupAnagrams(words);

    println!("{:?}", result);
}
