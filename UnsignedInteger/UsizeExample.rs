fn main() 
{
    let PointerSizeUnsignedInteger:usize = 42;
    println!("Pointer sized Unsigned Integer : {}", PointerSizeUnsignedInteger);

    let MaxUsize: usize = usize::MAX;
    println!("Maximum Usize Value: {}", MaxUsize);

    let minSize: usize = usize::MIN;
    println!("Minimum Usize value :{}", minSize);

    let some_value:i32 = 10;
    let pointer_to_value: * const i32 = &some_value;

    unsafe 
    {
        let PointAsUsize : usize = pointer_to_value as usize;
        println!("Pointer as usize : {}", PointAsUsize);
    }
}
