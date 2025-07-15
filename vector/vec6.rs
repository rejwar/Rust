fn main() {
    let mut Primes = vec![2,3,5];
    let MorePrimes = vec![7,11];

    Primes.extend(MorePrimes);
    println!("Extended vectors : {:?}", Primes);
}