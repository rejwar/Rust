// // fn takes_ownership(s: String) {
// //     println!("{}", s)
// // }

// // fn main() {
// //     let s1 = String::from("Hello");
// //     takes_ownership(s1);
// // }

// // fn TakesOwnership(s: String) {
// //     println!("{}", s);
// // }

// // fn main() {
// //     let s1 = String::from("Hello");
// //     TakesOwnership(s1);
// // }

// fn TakesOwnership(s: String) {
//     println!("{}", s);
// }

// fn main() {
//     let s1 = String::from("Hello");
//     TakesOwnership(s1);
// }

fn TakesOwnership(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("Hello");
    TakesOwnership(s1);
}
