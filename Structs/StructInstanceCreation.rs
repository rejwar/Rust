fn main() {
    struct  Coffee {
        price: f64,
        name: String,
        IsHot: bool,
    }

    let mocha: Coffee = Coffee{
        name: String ::from("Mocha"),
        price: 4.99,
        IsHot: false,
    };

    println!(
        "My {} this morning cost {} . It is {}  that it was hot",
        mocha.name, mocha.price , mocha.IsHot
    );

    let favoriteCoffee: String = mocha.name;
}
