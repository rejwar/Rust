fn main() {
    let mut Fruits = vec!["Apple" , "Banana" , "Cherry"];
    Fruits.insert(1, "Mango");
    println!("After insert {:?}", Fruits);

    Fruits.remove(2);
    println!("After remove : {:?}", Fruits);
}