fn Main() {
    let Original = String::from("Rustacean");

    let FirstClone = Original.clone(); // First clone
    let SecondClone = FirstClone.clone(); // Second clone

    println!("Original: {}, Clone: {}", Original, SecondClone);
}
