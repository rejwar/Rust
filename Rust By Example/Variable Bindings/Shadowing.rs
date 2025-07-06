fn main() {
    let x =5;
    println!("Before shadowing , x :{}", x);

    let x = x+10;
    println!("After shadowing x :{}",x);
}
