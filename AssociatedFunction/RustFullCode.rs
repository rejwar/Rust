use std::fmt::Debug;
use std::fmt::Display;

// ১. Trait define ও implement
trait Greet {
    fn say_hello(&self) -> String;
}

struct Person;

impl Greet for Person {
    fn say_hello(&self) -> String {
        String::from("Hello!")
    }
}

// ২. Trait with default method
trait DefaultGreet {
    fn say_hello(&self) -> String {
        String::from("Hi there!")
    }
}

struct Robot;

impl DefaultGreet for Robot {} // default method ব্যবহার করবে

// ৩. Trait as function parameter
fn greet_user(item: &impl Greet) {
    println!("{}", item.say_hello());
}

// ৪. Struct with lifetime
struct Message<'a> {
    content: &'a str,
}

// ৫. Function with lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ৬. Generic function
fn identity<T>(value: T) -> T {
    value
}

// ৭. Struct with generic type
struct Point<T> {
    x: T,
    y: T,
}

// ৮. Enum with generic type
enum MyOption<T> {
    Some(T),
    None,
}

// ৯. Method with trait bound
fn print_debug<T: Debug>(item: T) {
    println!("{:?}", item);
}

// ১০. Trait bound with multiple traits
fn clone_and_show<T: Clone + Display>(item: T) {
    let copy = item.clone();
    println!("{}", copy);
}

fn main() {
    // ১. Trait implement ব্যবহার
    let p = Person;
    println!("{}", p.say_hello());

    // ২. Default trait method ব্যবহার
    let r = Robot;
    println!("{}", r.say_hello());

    // ৩. Trait as parameter
    greet_user(&p);

    // ৪. Struct with lifetime
    let msg = Message {
        content: "Rust is awesome!",
    };
    println!("Message: {}", msg.content);

    // ৫. Lifetime function
    let s1 = "short";
    let s2 = "longer string";
    println!("Longest: {}", longest(s1, s2));

    // ৬. Generic function
    println!("Identity int: {}", identity(42));
    println!("Identity str: {}", identity("Hello"));

    // ৭. Generic struct
    let point_int = Point { x: 10, y: 20 };
    let point_float = Point { x: 1.5, y: 2.5 };
    println!("Point int: ({}, {})", point_int.x, point_int.y);
    println!("Point float: ({}, {})", point_float.x, point_float.y);

    // ৮. Enum with generic
    let opt1: MyOption<i32> = MyOption::Some(5);
    let opt2: MyOption<&str> = MyOption::None;
    match opt1 {
        MyOption::Some(val) => println!("Option has value: {}", val),
        MyOption::None => println!("Option is None"),
    }
    match opt2 {
        MyOption::Some(val) => println!("Option has value: {}", val),
        MyOption::None => println!("Option is None"),
    }

    // ৯. Trait bound method
    print_debug(vec![1, 2, 3]);

    // ১০. Multiple trait bound
    clone_and_show(String::from("Rust Generics"));
}
