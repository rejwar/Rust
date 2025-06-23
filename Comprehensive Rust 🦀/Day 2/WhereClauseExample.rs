fn PrintSum<T>(a: T , b: T) 

where 
    T: std::ops::Add<Output = T> + std::fmt::Display,

    {
            println!("Sum = {}" , a+b);
    }

    fn main() {
        PrintSum(5, 7);
    }
