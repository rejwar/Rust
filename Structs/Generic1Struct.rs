struct  Container<T> {
    Value: T ,
}

fn main() {
    let IntegerContainer: Container<i32> = Container { Value : 100};
    let StringContainer: Container<String> = Container{Value: String::from("Rust")};

    println!("Integer : {} " , IntegerContainer.Value);
    println!("String : {}" , StringContainer.Value);
}
