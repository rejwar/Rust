fn main() {
    let x =5;

    let x = if x > 0 {x +10} else {x-10};
    println!("After shadowing x {}",x);
}
