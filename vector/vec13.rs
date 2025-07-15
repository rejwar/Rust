use core::num;

fn main() {
    let numbers = vec![10,20,30,40,50];

    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled {:?}", doubled);

    let filtered: Vec<_> = numbers.into_iter().filter(|x|x % 20 != 0).collect();
    println!("Filterd {:?}", filtered);
}
