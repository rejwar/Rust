// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.lne() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len {
        y
    }
}
