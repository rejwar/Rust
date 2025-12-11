fn main() {
    let iter = get_iter();
    for i in iter {
        println!("{}", i);
    }
}

fn get_iter() -> impl Iterator<Item = i32> {
    let v = vec![1, 2, 3, 4];
    v.into_iter()
}
