fn main() {
    let mut seasons : [&str; 4] = ["Spring" , "Summer" ,"Fall" , "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}" , seasons[2]);
    println!("{}" ,true);
    println!("{:#?}",seasons);
}
