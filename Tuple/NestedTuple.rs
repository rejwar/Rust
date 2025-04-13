fn NestedTupleExample() {
    let data:(&str) (&str , i32)  = (("Rust , 2025") , ("Version" , 4.5));

    println!("{} -{}", (data.0).0,(data.0).1);
    println!("{} -{}", (data.1).0 , (data.1).1);
}

fn main()
{
    NestedTupleExample();
}
