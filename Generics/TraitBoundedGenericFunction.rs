fn AddOne<T: std::ops::Add<Output = T>>(num:T) -> T {
    num + num
}

fn TraitBoundGeneric() {
    println!("{}" , AddOne(5));
    println!("{}" , AddOne(1.5));
}
