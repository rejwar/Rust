fn main() {
    let mut x =10;
    {
        let y = &x;
        println!("Inside scope x : {} , y: {}",x,y);
    }

    x =30;
    println!("Outside scope is , x {}", x);
}
