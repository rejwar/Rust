fn main() {
    let str1 = String::from("long String");
    let result;
    {
        let str2 = String::from("short");
        result = longest(&str1 , &str2);

    }

    println!("{}", result);
}