use std::io;
use rand::prelude::*;

fn Main() {
    let GuessList = ["Banana", "Apple", "Mango", "Orange", "Grapes"];
    let mut Rng = thread_rng();

    let Index = Rng.gen_range(0..GuessList.len());
    let RandomFruit = GuessList[Index];
    println!("Random Fruit: {}", RandomFruit);

    let mut Input = String::new();

    match io::stdin().read_line(&mut Input) {
        Ok(_) => {
            let FruitSelected = Input.trim().to_lowercase();
            println!("Fruit Selected: {}", FruitSelected);
            
            if FruitSelected == RandomFruit.to_lowercase() {
                println!("Correct Guess!");
            } else {
                println!("Wrong Guess! The Fruit Was {}", RandomFruit);
            }
        }
        Err(Error) => {
            println!("Error: {}", Error);
        }
    }
}
