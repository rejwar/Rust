macro_rules! create_vec {
    ($type:ty) => {
        let my_vec: Vec<$type> = Vec::new();
        println!("A new vector of type {} has been created.", stringify!($type));
    };
}

fn main() {
    create_vec!(i32);
    create_vec!(String);
}