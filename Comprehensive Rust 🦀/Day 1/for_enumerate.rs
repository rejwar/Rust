fn main() {
    let items = ["Apple" , "Banana" , "Orange"];

    for (index , item)  in items.iter().enumerate(){
        println!("{} , {}", index, item);
    }
}
