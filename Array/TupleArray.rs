fn TupleArrayExample() {
    let points = [(1,2), (3,4) , (5,6)];

    for (x,y) in points.iter() 
    {
        println!("Point: ({}, {})", x,y);
    }
}

fn main() 
{
    TupleArrayExample();
}
