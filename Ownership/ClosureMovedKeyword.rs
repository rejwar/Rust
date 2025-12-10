fn main{
    let data = String::from("Sensitive  Data");

    let consume_data = move || {
        println!("Data inside closure {}", data);
    };

    consume_data();
}