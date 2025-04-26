fn main() {
    let Colors: [&str; 3] = ["Red", "Green", "Blue"];

    println!("Colors: {:?}", Colors);
    println!("First color: {}", Colors[0]);
    println!("Second color: {}", Colors[1]);
    println!("Third color: {}", Colors[2]);
    println!("Number of colors: {}", Colors.len());
    println!("Colors in reverse order: {:?}", Colors.iter().rev().collect::<Vec<_>>());
    println!("Colors in uppercase: {:?}", Colors.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>());
    println!("Colors in lowercase: {:?}", Colors.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Colors in sorted order: {:?}", Colors.iter().map(|s| s.to_string()).collect::<Vec<_>>());
}
