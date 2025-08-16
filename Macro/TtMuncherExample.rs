macro_rules! print_age {
    () => {};

    ($name:literal , $age:expr , $($rest:tt)*) => {
        println! ("{} is {} year old ", $name , $age);
        print_age!($($rest)*);
    };
}

fn main() {
    print_age!("Alice" ,30 , "Bob" , 40 , "newbie" , 70,);
}