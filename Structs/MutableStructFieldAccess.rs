struct  Counter  {
    Value: i32,
}


fn main() {
    let mut MyCounter: Counter = Counter { Value: 0 };
    MyCounter.Value += 5;
    println!("Updated Counter : {}" , MyCounter.Value);
    
}
