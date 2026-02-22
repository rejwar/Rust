use std::collections::HashSet;

// fn main() {
//     let mut fruits = HashSet::new();

//     fruits.insert("appple");
//     fruits.insert("Mango");
//     fruits.insert("appple");

//     if fruits.contains("Mango") {
//         println!(" Got the mango ");
//     }

//     println!("{:?}", fruits);

//     fruits.remove("Mango");
//     println!(" there are {} fruits in bucket ", fruits.len());
// }

// fn main() {
//     let names: Vec<&str> = vec!["Rahim", "Karim", "Rahmi", ""];

//     let mut unique_names = HashSet::new();

//     for name in names {
//         unique_names.insert(name);
//     }

//     println!(" Unique names are {:?}", unique_names);
// }

// fn main() {
//     let names = ["Karim", "Rahim", "Jabbar", "Bilkis", "Motoion", "Karim"];

//     let mut unique_names = HashSet::new();

//     for name in names {
//         unique_names.insert(name);
//     }

//     println!("Unique names {:?}", unique_names);
// }

fn main() {
    let names = ["Karim", "Rahim ", "Karim"];

    let mut unique_names = HashSet::new();

    for name in names {
        unique_names.insert(name);
    }

    println!(" Uniq name {:?}", names);
}
