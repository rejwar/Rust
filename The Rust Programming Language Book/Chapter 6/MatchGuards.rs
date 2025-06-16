// Question: How can we use `match` with conditions (guards) in enum pattern matching?

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn MatchWithGuards() {
    let temp = Temperature::Celsius(30);

    match temp {
        Temperature::Celsius(t) if t > 25 => println!("Hot Celsius: {}", t),
        Temperature::Celsius(t) => println!("Cool Celsius: {}", t),
        Temperature::Fahrenheit(t) if t > 77 => println!("Hot Fahrenheit: {}", t),
        Temperature::Fahrenheit(t) => println!("Cool Fahrenheit: {}", t),
    }
}
