fn process<T>(item: T) 
    where 
    T: Display + Debug  +Clone, {
        println!("Item is {}", item);
        println!("Debug is {}", item);
    }