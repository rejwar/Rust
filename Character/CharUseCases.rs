// ফাইল: CharUseCases.rs
fn Main() {
    // টেক্সট প্রসেসিং
    CountVowels("Hello, World!");
    
    // প্যালিনড্রোম চেকিং
    let Word = "level";
    println!("Is '{}' palindrome? {}", Word, IsPalindrome(Word));
    
    // ক্যারেক্টার ফ্রিকোয়েন্সি
    CountFrequency("banana");
}

fn CountVowels(Text: &str) {
    let mut VowelCount = 0;
    
    for Character in Text.chars() {
        match Character.to_lowercase().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => VowelCount += 1,
            _ => (),
        }
    }
    
    println!("'{}' contains {} vowels", Text, VowelCount);
}

fn IsPalindrome(Text: &str) -> bool {
    let CharVec: Vec<char> = Text.chars().collect();
    let Length = CharVec.len();
    
    for i in 0..Length/2 {
        if CharVec[i] != CharVec[Length - 1 - i] {
            return false;
        }
    }
    
    true
}

fn CountFrequency(Text: &str) {
    let mut FrequencyMap = std::collections::HashMap::new();
    
    for Character in Text.chars() {
        let Count = FrequencyMap.entry(Character).or_insert(0);
        *Count += 1;
    }
    
    println!("Character frequencies in '{}':", Text);
    for (Character, Count) in FrequencyMap {
        println!("{}: {}", Character, Count);
    }
}
