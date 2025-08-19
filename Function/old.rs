fn main() {
    greet("Rifat" , 25);
}

fn greet(name: &str , age: i32) {
    println!("hello , {}! you are {} years old " , name, age);
}