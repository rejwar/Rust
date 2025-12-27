// // fn main() {
// //     println!("Hello, world!");
// // }
// // //
// let x = 5;
// let y = Box::new(x);

// println!("{}", x == *y);
// fn UseBoxDeref() {
//     let x = Box::new(7);
//     let y = *x;
//     println!("y = {}", y);
// }
fn UseBoxMove() {
    let a = Box::new(5);
    let b = a;
    println!(" VAlue is b{}", b);
}

fn main() {
    UseBoxMove();
}
