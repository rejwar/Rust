fn PrintUnicodeValue()
{
    let Ch:char = 'A';

    println!("'{}' as unicode: U+{:04X}" , Ch , Ch as u32 );
}

fn main()

{
    PrintUnicodeValue();
}
