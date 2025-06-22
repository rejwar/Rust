fn main() {
    let opt: Option<i32> = Some(42);

    if let  Some(value) =  opt {
        println!("Got a value : {}", value);
    }else {
        println!("No value  found");
    }

}
