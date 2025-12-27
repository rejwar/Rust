fn UseBoxMemoryMosel() {
    let x = 10;
    let bx = Box::new(20);

    println!(" Stack value {}", x);
    println!(" Heap value via Box bx = {}", bx);
}

fn main() {
    UseBoxMemoryMosel();
}
