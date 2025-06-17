enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    let today = Day::Friday;

    match today {
        Day::Friday => println!("It's almost weekend!"),
        Day::Saturday | Day::Sunday => println!("It's weekend!"),
        _ => println!("It's a weekday."),
    }
}
