fn main() {
    let mut CurrentMeal: String = String::new();
    CurrentMeal = AddFlour(&CurrentMeal);
    ShowMyMeal(&CurrentMeal);
}

fn AddFlour(meal : &String) {
    meal.push_str(String"Add Flour");
}

fn ShowMyMeal (meal : &String) {
    println!("Meal steps : {}", meal);
}
