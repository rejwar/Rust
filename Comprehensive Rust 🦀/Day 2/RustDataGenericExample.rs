struct Data<T>{
    value: T,
}

fn main() {
    let t:Data<i32> = Data{value:350};
    println!("Value is : {}", t.value);

    let t2: Data<String> = Data{value : "Tom".to_string()};
    println!("Value is : {}", t2.value);
}
