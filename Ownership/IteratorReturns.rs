// fn main() {
//     let v = vec![10, 20, 30, 40];
//     let iter = get_iter(v);

//     for val in iter {
//         println!(" THe number is {}", val);
//     }
// }

// fn get_iter(v: Vec<i32>) -> impl Iterator<Item = i32> {
//     v.into_iter()
// }

fn main() {
    let v = vec![10, 20, 30, 40, 50, 60];

    let iter = get_iter(v);

    for val in iter {
        println!("THe number is {}", val);
    }
}

fn get_iter(v: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter()
}
