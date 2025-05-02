#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum Beans {
    Black,
    Pinto,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { Meat: Meat, Beans: Beans },
    Bowl { Meat: Meat, Beans: Beans },
    VeganPlate,
}

fn Main() {
    let Lunch = RestaurantItem::Burrito { 
        Meat: Meat::Steak, 
        Beans: Beans::Black,
    };

    let Dinner = RestaurantItem::Bowl { 
        Meat: Meat::Steak,
        Beans: Beans::Pinto,
    };

    let AbandonedMeal = RestaurantItem::VeganPlate;
    println!("Lunch was {:?} and dinner was {:?}", Lunch, Dinner);
    println!("Nobody ate {:?}", AbandonedMeal);
}
