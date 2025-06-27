

fn main() {
    let Values = vec![1,2,3,4,5,6];

    let evens: Vec<_> = Values.iter().filter(|&&x| x % 2 ==0).collect();
    println!("Evens in the rust {:?}", evens);
}
