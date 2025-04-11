fn Main() 
{
    let IsRustFun:bool = true;
    let IsLearningEasy:bool = false;

    let AndResult:bool = IsRustFun && IsLearningEasy;
    let OrResult:bool = IsRustFun || IsLearningEasy;
    let NotResult:bool =!IsRustFun;

    println!("Basic Booleans : {} , {}",IsRustFun, IsLearningEasy );
    println!("Logical AND: {}", AndResult);
    println!("Logical OR: {}", OrResult);
    println!("Logical NOT: {}", NotResult);

    if IsRustFun 
    {
        println!("Rust is fun Indeed:");

    }
    else
    {
        println!( "You might need to give RUST another chance");
    }

}

fn main() {
    Main();
}
