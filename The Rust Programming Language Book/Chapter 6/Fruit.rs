enum Fruit {
    Mango,
    Apple,
    Banana,
}

fn describe (f: Fruit) {
    match f {
        Fruit::Apple => println!("Sweet and juicy"),
        Fruit::Mango => println!("Yellow and juicy"),
        Fruit::Banana => println!("Soft and yellow"),
    }
}

fn main() {
    let my_fruit = Fruit::Apple;
    describe(my_fruit);
}
