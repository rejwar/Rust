fn main() {
    let x = 10;
    let print_x = || println!("{}", x);
    print_x();

    println!("x still here {}", x);
}
