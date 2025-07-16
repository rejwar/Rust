fn main() {
    let name = String::from("Tylor");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");


    println!("{}" , name);
    println!("{}", course);
    println!("{}", new_name);

    let str1 = "Hello";
    println!("{}", str1.bogus());
}