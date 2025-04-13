fn TupleArrayExample()
{
    let students = [("Alice" ,90), ("Bob" , 80) , ("Charlie" , 85)];

    for (name ,score) in students.iter() {
        println!("{} scored {}" , name, score );
    }
}

fn main() {
    TupleArrayExample();
}
