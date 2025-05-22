fn main() {
    let Fruits: [&str;3] = ["Apple"  , "Banana" , "Cherry"];

    let FirstFruit: &str = Fruits[0];
    let SecondFruit: &str = Fruits[1];

    println!("First Fruit is : {}", FirstFruit);
    println!("Second Fruit is : {}", SecondFruit); 
}
