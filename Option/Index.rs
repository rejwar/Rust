let arr = [1,2,3,4,5,6,7];

let idx = find_Odd(&arr ,3);

match idx {
    Some(i) => println!("Found at the number {}", i),
    None => println!("Not Funny"),
}