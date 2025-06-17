// Question: How to use pattern matching with condition (guard)?

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn MatchTemperature() {
    let temp = Temperature::Celsius(35);

    match temp {
        Temperature::Celsius(t) if t > 30 => println!("It's hot in Celsius: {}", t),
        Temperature::Celsius(t) => println!("Cool Celsius: {}", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("Hot in Fahrenheit: {}", t),
        Temperature::Fahrenheit(t) => println!("Cool Fahrenheit: {}", t),
    }
}
