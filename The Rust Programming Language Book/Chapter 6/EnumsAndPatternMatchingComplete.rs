// Question: How to use enums with pattern matching in a runnable program?

enum Direction {
    North,
    South,
    East,
    West,
}

fn MatchDirection() {
    let dir = Direction::East;

    match dir {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn UseMessage() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("The app will quit."),
        Message::Move { x, y } => println!("Move to position: ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

fn UseOptionEnum() {
    let some_value = Some(100);
    let no_value: Option<i32> = None;

    match some_value {
        Some(num) => println!("Got number: {}", num),
        None => println!("No value found"),
    }

    match no_value {
        Some(num) => println!("Won't print"),
        None => println!("Correct: nothing inside"),
    }
}

fn Divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn UseResultEnum() {
    match Divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match Divide(10, 0) {
        Ok(_) => println!("Will not happen"),
        Err(e) => println!("Caught error: {}", e),
    }
}

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

fn UseIfLet() {
    let value = Some("Rust");

    if let Some(lang) = value {
        println!("You're learning {}", lang);
    }
}

fn main() {
    MatchDirection();        // Call for enum Direction
    UseMessage();            // Call for enum with data
    UseOptionEnum();         // Call for Option enum
    UseResultEnum();         // Call for Result enum
    MatchTemperature();      // Call for enum with guard
    UseIfLet();              // Call for if let match
}
