fn main() {
    let numbers = vec![1,2,3,4,5,6,7];

    let evens: Vec<_> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!(" Evens is {:.?}", evens);
}