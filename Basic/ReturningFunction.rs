fn GetOperation(op: &str) -> fn(i32 , i32) -> i32
{


    match op{
        "add" => Add,
        "multiply" => Multiply,
        _=> |_,_| 0,
    }
}

fn Add(a: i32 , b:i32 ) -> i32{
    a+ b
}

fn Multiply (a:i32 , b: i32) -> i32{
    a*b
}

fn main()
{
    let operation = GetOperation("add");
    println!("Result: {}" , operation(10,5));
}

