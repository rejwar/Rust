fn main() {

    let food:&str  = "Pizza";

    println!("{}", food.len());

    let PizzaSlice:&str = &food[1..3];

    println!("{}", PizzaSlice.len());

}
