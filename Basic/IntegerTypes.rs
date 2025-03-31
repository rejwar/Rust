fn ShowIntegerTypes()
{
    let signed_interger: i32 =-10;
    let unsigned_interger: u32 = 100;
    let small_number: u8 = 255;
    let large_number: i64 = 1_000_000;

    println!("Signed Integer: {}" , signed_interger);
    println!("UnSigned Integer: {}" , unsigned_interger);
    println!("Small Number :{}", small_number);
    println!("Large Number :{}", large_number);
}

fn main()
{
    ShowIntegerTypes();
}
