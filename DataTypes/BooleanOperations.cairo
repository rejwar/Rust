fn main() {
    println!("{}" , !true);
    println!("{}" , !false);

    let age:i32 = 13;
    let can_see_rated_r_movie:bool = age >= 17;
    println!("{}" , can_see_rated_r_movie);

    let cannot_see_rated_r_movie:bool = !can_see_rated_r_movie;
    println!("{}" , cannot_see_rated_r_movie);
    let is_teenager:bool = age >= 13 && age <= 19;
    println!("{}" , is_teenager);
    let is_child:bool = age < 13;
    println!("{}" , is_child);
    let is_adult:bool = age > 19;
    println!("{}" , is_adult);
    let is_teenager_or_child:bool = is_teenager || is_child;
    println!("{}" , is_teenager_or_child);
    let is_teenager_and_adult:bool = is_teenager && is_adult;
    println!("{}" , is_teenager_and_adult);
    let is_teenager_xor_adult:bool = is_teenager ^ is_adult;
    println!("{}" , is_teenager_xor_adult);
    let is_teenager_xor_child:bool = is_teenager ^ is_child;
    println!("{}" , is_teenager_xor_child);
    let is_teenager_and_child:bool = is_teenager && is_child;
    println!("{}" , is_teenager_and_child);
    let is_teenager_or_adult:bool = is_teenager || is_adult;
    println!("{}" , is_teenager_or_adult);
    let is_teenager_or_child_or_adult:bool = is_teenager || is_child || is_adult;
    println!("{}" , is_teenager_or_child_or_adult);
    let is_teenager_and_child_and_adult:bool = is_teenager && is_child && is_adult;
    println!("{}" , is_teenager_and_child_and_adult);
    let is_teenager_and_child_or_adult:bool = is_teenager && is_child || is_adult;
}
