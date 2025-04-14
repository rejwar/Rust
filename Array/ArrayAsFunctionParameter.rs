fn ArrayAsFunction(data: [i32 ;3]) {
    for val in data.iter() {
        println!("{}",val);
    }

    fn main()
    { let arr = [7,8 ,9];
        ArrayAsFunction(arr);
    }
}
