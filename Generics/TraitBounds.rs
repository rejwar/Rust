use std::ffi::os_str::Display;

fn print<T: std::fmt::Debug + Clone > (item: T){
    println!("{:?}", item);
}

fn complex< T , U> (t: T , u: U) {
    where T: Display + Clone, 
    U: Clone + Debug {}
}