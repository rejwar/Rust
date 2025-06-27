fn main() {
    let data = vec![10,2,3,4,5];

    for val in data.iter() {
        println!(" value : {}" ,val);
    }

    println!(" Original data is still usable {:?}",data);
}

