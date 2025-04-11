use std::isize;

fn main() 
{
    let PointSizeInteger: isize = 42;
    println!("Pointer Size Integer: {}", PointSizeInteger);

    let AnotherSizeInteger: isize = -100;
    println!("Another Pointer Size Integer: {}", AnotherSizeInteger);

    let MaxIsize: isize = isize::MAX;
    println!("Maximum Point size integer : {}", MaxIsize);

    let MinIsize:isize = isize::MIN;
    println!("Minimun Point size Integer: {}", MinIsize);
    

    let some_value:i32 = 10;
    let PointerToValue: *const i32 = &some_value;

    unsafe {
        let PointerAsSize: isize = PointerToValue as isize;
        println!("Poointe as i size :{}", PointerAsSize);
    }
}
