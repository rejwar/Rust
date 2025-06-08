fn main() {
    let ImmutableVar:i32 =10;
    let mut MutableVar:i32 = 25;
    
    MutableVar += 10;

    println!("Immnutable Number {}", ImmutableVar);
    println!("Mutable Number {}", MutableVar);

}
