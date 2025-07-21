fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8];

    let squared_events: Vec<_> = numbers.into_iter().filter(|x| x % 2 ==0 ).map(|x| x *2).collect();

    println!("Squared even number {:.?}", squared_events);
}