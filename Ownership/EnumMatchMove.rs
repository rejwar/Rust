// enum Data {
//     Text(String),
// }

// fn main() {
//     let d = Data::Text(String::from("Inside"));

//     if let Data::Text(s) = d {
//         println!("Extractd {}", s);
//     };
// }
 enum Data {
    Text(String),
 }

 fn main() {
    let d = Data::Text(String::from("Inside"));

    if let Data::Text(s) = {
        println!("Extracted {}",s);
    };
 }