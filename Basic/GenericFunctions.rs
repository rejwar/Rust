fn Swap<T>( a: T, b: T) -> (T, T)
{
    (b,a)
}

fn main()
{
    let (x,y) = Swap(5,10);

    println!("Swapped: {} {}",x,y);

    let (a,b) = Swap("Hello" , "World");
    println!("Swapped: {} {}" , a,b);
}
