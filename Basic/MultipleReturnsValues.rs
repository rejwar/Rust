fn DividAndRemainder(a: i32 , b: i32) -> (i32, i32)
{
    (a/b , a%b)
}

fn main()
{
let (quotient , remainder) = DividAndRemainder(10, 3);
println!("Quotient: {} , Remainder: {}" , quotient, remainder);
}
