fn main() {
    let mut x =10;
println!("Before freezing , x {}", x);

let y = &x;
println!("During freezing , x: {} , y: {}",x,y);

x =30;
println!("After freezing x {}",x);
}
