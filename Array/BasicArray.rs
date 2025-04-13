fn main() 
{
    let Numbers: [i32;5] = [1,2,3,4,5];

    println!("Full Array: {:?}" , Numbers);

    println!("First Element: {}" , Numbers[0]);
    println!("THrid Element: {}" , Numbers[2]);

    println!("Array length: {}" , Numbers.len());

    let SameValue: [i32 ; 5] = [0;5];
    println!("Same value array  {:?}", SameValue);
}
