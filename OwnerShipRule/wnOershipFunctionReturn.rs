

fn main() 
{
    let Data:String = createData();
    println!("{}", Data);
}
fn createData() -> String
{
    String::from("Owned By function")
}

