fn  Add<T: std::ops::Add<Output = T>> (a: T , b: T) -> T {
    a +b
}

fn main() {
    println!("{}", Add(10,20));
    println!("{}",Add(1.4, 2.6));
}
