fn LoopWithTuple() 
{
    let items = ["apple" , "banana", "cherry"];

    for (index , item) in items.iter().enumerate(){
        println!("Index: {} , Item:{}", index,item);
    }
}

fn main()
{
    LoopWithTuple();
}
