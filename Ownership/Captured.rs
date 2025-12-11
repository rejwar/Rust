// fn main() {
//     let s = String::from("Captured");

//     let c = move || {
//         println!("{}", s);
//     };

//     c();
//     c
// }
fn main() {
    let s = String::from("Captired ");

    let c = move || {
        println!("{}", s);
    };
    c();
}
