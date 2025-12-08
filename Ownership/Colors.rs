// fn main() {
//     let colors: Vec<String> = vec![
//         String::from("Red"),
//         String::from("Green"),
//         String::from("Blue"),
//     ];

//     println!("--- Start loop");

//     for color in colors.iter() {
//         println!(" Looking at {}", color);
//     }

//     println!("---------- End Loop------");

//     println!("My color are safe  :{:?}", colors);
// }
fn main() {
    let colors: Vec<String> = vec![
        String::from("Red"),
        String::from("Green"),
        String::from("Blue"),
    ];

    println!("------ Start Loop----");

    for color in colors.iter() {
        println!("LOOKING AT {}", color);
    }

    println!("------- End Loop ____-");

    println!("My color are safe {:?}", colors);
}
