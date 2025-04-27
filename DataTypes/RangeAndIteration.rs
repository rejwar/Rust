use std::collections::btree_map::Range;

fn main() {
    let MonthDays = 1..31;

    println!("{:?}", MonthDays);

    let MonthDays =1..=31;
    println!("{MonthDays:?}");


    for number  in MonthDays{
        println!("{number}");
    }

    let letters = 'b'..'h';

    for letter in letters {
        println!("{}" , letter);
    }

    let colors = ["red" , "Green" , "Yello"];

    for color in colors{
        println!("{} is a great color! ", color);
    }
}
