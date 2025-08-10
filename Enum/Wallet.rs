fn main() {
    let wallet: Option<u32> = Some(500);
    let empty_wallet: Option<u32> = None;

    match wallet {
        Some(amount) => println!("U have take {}", amount),
        None => println!("No money"),
    }

    match empty_wallet {
        Some(amount) => println!("U have amount of taka {}", amount),
        None => println!("No money"),
    }
}