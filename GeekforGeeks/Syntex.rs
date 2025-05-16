fn main() {
    let gfg = ("cp" , "Algo" , "Dsa" , "Faang", "Data structure");

    println!("The tuple is: {:?}", gfg);

    println!("The first element is: {}", gfg.0);
    println!("The second element is: {}", gfg.1);
    println!("The third element is: {}", gfg.2);
    println!("The fourth element is: {}", gfg.3);
    println!("The fifth element is: {}", gfg.4);
    println!("The length of the tuple is: {}", std::mem::size_of_val(&gfg));
}
