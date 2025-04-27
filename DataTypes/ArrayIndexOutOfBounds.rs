fn main() {
    let seasons: [&str; 3] = [ "Spring" , "Fall" , "Winter "];
    let first:&str = seasons[0];
    let second: &str = seasons[0];
    println!("The first season is {first} and the second season is {second}");

    println!("{}" , seasons[100]);
}
