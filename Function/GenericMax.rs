fn Max<T:PartialOrd> (a: T, b:T) -> T {
    if a>b {
        a 
    } else {
        b
    }
}

fn main()  {
 println!("Max : {}" , Max (10,20));
 println!("Max: {}" , Max (3.14,2.71));
}
