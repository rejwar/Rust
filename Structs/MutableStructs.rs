struct Counter{
    value: i32,
}

fn UpdateCounter() {
    let mut c = Counter {value:0};
    c.value +=5;

    println!("counter: {}" , c.value);
}

fn main() {
    UpdateCounter();
}
