// // fn longest(x: &str, y: &str) -> &str {
// //     if x.len() > y.lne() {
// //         x
// //     } else {
// //         y
// //     }
// // }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len {
//         y
//     }
// }

fn main() {
    let s1 = String::from("Abcd");
    let s2 = String::from("xyz");

    let result = longest(s1.as_str(), s2.as_str());

    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
