enum Season {
    Summer,
    Winter,
    Rainy,
}


fn main() {
    let current_season = Season::Rainy;

    match current_season {
        Season::Rainy => println!("Its rainy season"),
        Season::Summer => println!("Its winter season"),
        Season::Winter => println!("Its summer season"),
    }
}