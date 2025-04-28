fn MatchWithGuardExample(pair: (i32 , i32)) {
    match pair {
        (x,y) if x ==y => println!("Both coordinates are equale"),
        (x,y) if x+y == 0 => println!("Sum is zero"),
        (x,y) => println!("Different values :({} ,{})", x,y),
    }
}

fn main() {
    MatchWithGuardExample((3,3));
    MatchWithGuardExample((5, -5));
    MatchWithGuardExample((2,7));
}
