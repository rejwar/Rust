fn main()
{
    let Num:f64 = 2.6;

    println!("Original: {}", Num);
    println!("Rounded:{}", Num.round());
    println!("Floor:{}", Num.floor());
    println!("Ceil:{} ", Num.ceil());
    println!("truncated: {}", Num.trunc());
}
