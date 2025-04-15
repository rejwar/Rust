fn UpdateCounter (counter :&mut i32) {
    *counter +=1;
}

fn main() {
    let mut count =0;
    UpdateCounter(&mut count);
    println!("Count : {}", count);
}
